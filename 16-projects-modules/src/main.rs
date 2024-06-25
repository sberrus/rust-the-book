// Esta línea importa el Asparagus type dentro de main
use crate::garden::vegetables::Asparagus;
use crate::garden::vegetables::Pineapple;
use crate::garden::vegetables::Tomato;

// importamos el módulo garden como público
mod garden;

fn main() {
    let asparagus: Asparagus = Asparagus {};
    let pineapple: Pineapple = Pineapple {};
    let tomato: Tomato = Tomato {};
    println!("I'm growing {asparagus:?}, {pineapple:?} and {tomato:?}!");
}
