use std:: collections::VecDeque;
use colored::*;

struct RedTrenes {
   estaciones: Vec<String>,
   adyacencia: Vec<Vec<usize>>,
}



pub fn ejecutar(){
    //llamamos a la funcion para crear vías entre estaciones
   red.agregar_via(auroria,  velstrom);
   red.agregar_via(velstrom, nexara);
   red.agregar_via(nexara,   dralion);
   red.agregar_via(dralion,  korveth);
   red.agregar_via(korveth,  myrenth);
   red.agregar_via(myrenth,  caldrix);
   red.agregar_via(caldrix,  zentova);
   red.agregar_via(zentova,  auroria);


   //Pyloran es nuestro centro de conexiones, tiene vías a casi todas las estaciones
   red.agregar_via(pyloran, auroria);
   red.agregar_via(pyloran, nexara);
   red.agregar_via(pyloran, korveth);
   red.agregar_via(pyloran, caldrix);


   //thornex es una estacion apartada, de las demas estaciones solo tiene vias a dralion y velstrom
   red.agregar_via(thornex, dralion);
   red.agregar_via(thornex, velstrom);

   red.mostrar_diagrama(None);

}


// Conecta dos estaciones mediante una vía.
    // Como el grafo es NO DIRIGIDO, la conexión se registra en ambos sentidos
    // (si A llega a B, entonces B también llega a A).

    fn agregar_via(&mut self, a: usize, b: usize) {
       // A puede llegar a B.
       self.adyacencia[a].push(b);
       // B puede llegar a A.
       self.adyacencia[b].push(a);
   }

fn mostrar_diagrama(&self, ruta: Option<&Vec<usize>>) {
       // let coords: Este trozo de código son las coordenadas de cada estación para luego dibujar el
       // diagrama de la red, se asignan manualmente para que quede visualmente bien, no es
       // parte del algoritmo de grafos ni nada por el estilo, es solo para mostrar el diagrama
       // de la red de trenes con las estaciones en posiciones fijas.
       let coords: [(i32, i32); 10] = [
           (13,  5),  // 0 Auroria
           (32,  2),  // 1 Velstrom
           (51,  5),  // 2 Nexara
           (54, 13),  // 3 Dralion
           (44, 20),  // 4 Korveth
           (32, 23),  // 5 Myrenth
           (13, 20),  // 6 Caldrix
           ( 4, 13),  // 7 Zentova
           (32, 13),  // 8 Pyloran
           (59,  3),  // 9 Thornex
       ];


       //Esto crea el recuadro donde todo el diagrama se crea.
       const FILAS: usize = 27;
       const COLS:  usize = 70;
       // Borde: ║ + espacio + COLS celdas + espacio + ║  → ancho visual fijo
       let mut canvas: Vec<Vec<(char, u8)>> = vec![vec![(' ', 0); COLS]; FILAS];


       // convierte el camino encontrado por BFS en un conjunto de pares (aristas),
       // para poder identificar rápidamente cuáles vías pintar de otro color.
       let ruta_aristas: std::collections::HashSet<(usize,usize)> = ruta
           .map(|r| r.windows(2).map(|w| (w[0].min(w[1]), w[0].max(w[1]))).collect())
           .unwrap_or_default();
       // guarda los índices de las estaciones que forman parte de la ruta,
       // para luego asignarles un color distinto al dibujarlas.
       let ruta_nodos: std::collections::HashSet<usize> = ruta
           .map(|r| r.iter().cloned().collect())
     .unwrap_or_default();
       // guarda el índice de la estación de origen y destino,
       // para pintarlas de un color distinto (rojo) más adelante.
       let origen_idx  = ruta.map(|r| r[0]);
       let destino_idx = ruta.map(|r| *r.last().unwrap());


       //Trozo de código que nos sirve para poder ser llamado para dibujar las vías entre estaciones,
       // dependiendo de la pendiente de la línea se dibuja con un carácter u otro para
       // que quede visualmente más claro.
       let trazar = |canvas: &mut Vec<Vec<(char,u8)>>,
                         ax: i32, ay: i32, bx: i32, by: i32, color: u8| {
           let dx = bx - ax;
           let dy = by - ay;
           let steps = dx.abs().max(dy.abs());
           if steps == 0 { return; }
           for t in 1..steps {
               let x = (ax + dx * t / steps) as usize;
               let y = (ay + dy * t / steps) as usize;
               if x >= COLS || y >= FILAS { continue; }
               if canvas[y][x].1 >= 2 { continue; }
               let slope = if dx == 0 { 99.0f32 } else { (dy as f32 / dx as f32).abs() };
               let ch = if slope < 0.35 {
                   '─'
               } else if slope > 2.5 {
                   '│'
               } else if (dx > 0 && dy > 0) || (dx < 0 && dy < 0) {
                   '╲'
               } else {
                   '╱'
               };
               canvas[y][x] = (ch, color);
           }
       };


       //Aquí se llama a trazar para dibujar las vías entre estaciones,
       // se recorre la lista de adyacencia y se dibuja una línea entre
       // cada par de estaciones conectadas, si la vía está en la ruta resaltada
       // se dibuja con un color diferente.
       for i in 0..self.estaciones.len() {
           for &j in &self.adyacencia[i] {
 if j <= i { continue; }
               let (ax, ay) = (coords[i].0, coords[i].1);
               let (bx, by) = (coords[j].0, coords[j].1);
               let en_ruta = ruta_aristas.contains(&(i.min(j), i.max(j)));
               trazar(&mut canvas, ax, ay, bx, by, if en_ruta { 5 } else { 1 });
           }
       }


       //Aqui es donde se hacen los resaltados de colores en diagrama se decide que
       // color tendrá cada estación al iniciar el programa e igual buscar rutas.
       for (i, nombre) in self.estaciones.iter().enumerate() {
           let (cx, cy) = (coords[i].0 as usize, coords[i].1 as usize);
           let color: u8 = if Some(i) == origen_idx || Some(i) == destino_idx {
               4  // rojo
           } else if ruta_nodos.contains(&i) {
               3  // amarillo — intermedia
           } else if i == 8 {
               6  // verde — Centro de Conexiones (Pyloran)
           } else {
               2  // azul — normal
           };
           canvas[cy][cx] = ('●', color);
           let etiqueta = format!("[{}]{}", i, nombre);
           for (k, ch) in etiqueta.chars().enumerate() {
               let px = cx + 1 + k;
               if px < COLS { canvas[cy][px] = (ch, color); }
           }
       }
      
       //Se crea el recuadro visualmente que se ve el diagrama la parte de arriba y
       //  tambien la primera línea que encierra el diagrama
       // ── Cabecera (ancho fijo 72: ║ + 70 + ║) ────────────────
       let sep = "═".repeat(72);
       let sep2 = "─".repeat(72);
       println!();
       println!("{}", format!("╔{}╗", sep).yellow());
       // Título centrado sobre 70 caracteres ASCII — sin Unicode dentro → format! funciona bien
       let titulo = format!("{:^70}", "REDRAIL — DIAGRAMA DE RED");
       println!("{}", format!("║{}  ║", titulo).yellow());
       if let Some(r) = ruta {
  let o = &self.estaciones[r[0]];
           let d = &self.estaciones[*r.last().unwrap()];
           // Línea de ruta: print! directo para no romper el ancho
           // "║  Ruta resaltada: " = 19 chars visibles
           // " → " = 3 chars visibles
           // Total fijo visible dentro de los 70: 19 + o + 3 + d
           let contenido = format!("  Ruta resaltada: {} → {}", o, d);
           let vis = contenido.chars().count();
           let relleno = if 70 > vis { 70 - vis } else { 0 };
           print!("{}", "║".yellow());
           print!("{}", format!("  Ruta resaltada: "));
           print!("{}", o.bright_red().bold());
           print!("{}", " → ");
           print!("{}", d.bright_red().bold());
           print!("{}", " ".repeat(relleno));
           println!("{}", "║".yellow());
       }
       println!("{}", format!("╠{}╣", sep).yellow());


       //Aqui definimos los colores a utiliza para las estaciones y vias y
       //a demas de crear una le las lineas que encierra el diagrama
       // ── Filas del canvas ─────────────────────────────────────
       for fila in &canvas {
           print!("{}", "║ ".yellow());
           for &(ch, color) in fila {
               let s = ch.to_string();
               let out = match color {
                   1 => s.bright_black(),
                   2 => s.bright_blue().bold(),
                   3 => s.bright_yellow().bold(),
                   4 => s.bright_red().bold(),
                   5 => s.bright_yellow().bold(),
                   6 => s.bright_green().bold(),
                   _ => s.normal(),
               };
               print!("{}", out);
           }
           println!("{}", " ║".yellow());
       }


 //Aqui se crea la parta donde muestra que descripción del diagrama.
       // ── Leyenda ──────────────────────────────────────────────
       println!("{}", format!("╠{}╣", sep2).yellow());
       print!("{}", "║  ".yellow());
       print!("{}", "●".bright_blue().bold());   print!(" Estación normal   ");
       print!("{}", "●".bright_green().bold());  print!(" Centro de Conexiones(Pyloran)   ");
       print!("{}", "●".bright_red().bold());    print!(" Origen/Destino   ");
       println!();
       print!("{}", "   ──".bright_yellow().bold()); print!(" Vía en ruta");
       println!("{}", "                 ".yellow());
       println!("{}", format!("╚{}╝", sep).yellow());
       println!();
   }   

