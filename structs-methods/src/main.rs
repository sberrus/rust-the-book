/*
 *  Structs and methods
 *
 *  A los metodos son similares a las funciones, de por si también cuenta con la keyword fn para
 *  declararlas, las principales diferencias entre una función y un método es que el método forma
 *  parte del contexto de los structs, los enums y los traits.
 *
 *  Por otro lado, el primer argumento siempre es `self` siendo este la referencia de la instancia
 *  la cual dicho método forma parte.
 * */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// extendemos el struct usando la kw impl para integrar el método area
impl Rectangle {
    fn area (&self) -> u32 {
        /*
         * En este caso, podemos ver que con el self, podemos acceder a las propiedades de width y
         * height del struc Rectangle y usar los valores que tenga la instancia.
         * */
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    /*
     *  Como el método area() se ha extendido al struct Rectangle, podemos hacer uso de este para
     *  poder crear un método que compartiran todas las instancias del struct Rectangle.
     * */
    println!("El area del rectangulo es: {}",rect1.area());
}
