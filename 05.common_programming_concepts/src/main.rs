// TIPOS DE DATOS EN RUST
fn main() {
    /*
     Los números como en alugnos lenguajes fuertemente tipados, se usa i para para los números con signo y u para los números
     exclusivamente positivos positivos. Ya despues se le asigna el tamaño de bits del espacio en memoria.
    */

    let _solo_positivo: u8 = 15; // positivo
    let _positivo_negativo: i8 = -15; // positivo y negativo
    let _flotante: f64 = 3.5235; // tipo punto flotante, por defecto 64 bits

    // booleanos

    let _t = false;
    let _x: bool = true;

    let emoji = "😊";

    dbg!(emoji);
}
