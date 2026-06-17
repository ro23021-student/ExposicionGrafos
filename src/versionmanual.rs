use std:: collections::VecDeque;
use colored::*;

pub fn ejecutar(){
    
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


