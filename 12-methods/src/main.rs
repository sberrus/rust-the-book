/*
 *  Structs and methods
 *
 *  A los metodos son similares a las funciones, de por si también cuenta con la keyword fn para
 *  declararlas, las principales diferencias entre una función y un método es que el método forma
 *  parte del contexto de los structs, los enums y los traits.
 *
 *  Por otro lado, el primer argumento siempre es `self` siendo este la referencia de la instancia
 *  la cual dicho método forma parte.
 *
 *  ASSOCIATED FUNCTIONS
 *
 *  Todas las funciones que se encuentran dentro de un `impl` se les llama funciones asociadas
 *  `associated functions`.
 *
 * Dentro de los `impl` podemos definir funciones sin el `self` como primer argumento, estas no
 * necesitan para ser llamadas, estar instanciadas. Las podemos consumir directamente mediante el
 * uso de `::`. Sería una especie de método estático como suele definirse en otros lenguajes como
 * java o javascript, los cuales podemos llamar dicho método y capturar el dato sin necesidad de
 * tener que instanciar primero el struct.
 *
 * Usualmente, estas funciones suelen ser usadas para generar una nueva instancia del struct al que
 * pertenece. Sería un simil a la keyword `new` de otros lenguajes como java.
 *
 *
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

    // Podemos definir como nombre de metodo el mismo nombre de un campo del struct, siempre y
    // cuando no hagan conflicto los nombres de las propiedades o si no hay otro método con el
    // mismo nombre.
    fn width (&self){
        println!("Hello from Rectangle");
    }

    // Practica -> crear un método para que evalue si la instancia puede tener dentro otra
    // instancia de Rectangle, se buscará a evaluar si teniendo una instancia rect1.area() es mayor
    // a la instancia pasada como argumento al método y si su area es menor que el area de la
    // instancia que esta invocando al método .can_hold(&rect)
    //
    // Hint:
    //
    // can_hold(&self, rect: &Rectangle) -> bool 
    // self.area() > rect.area() -> true
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width  && self.width > other.width
    }

    // Construyendo una funcion asociada.
    /*
     *  Como observamos en este caso debemos devolver un tipo `Self` el cual indica que lo que
     *  devuelve es una instancia del objeto que esta referenciando.
     * */
    fn square(size: u32) -> Self {
        Self {
            width:size,
            height: size,
        }
    }
}

// Es válido tener multiples impl para la misma estructura.
impl Rectangle {
    fn print_something(&self){
        println!("Otro impl? :vvvv");
    }
}



fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // instanciando un cuadrado con la función asociada.
    let square1 = Rectangle::square(50);

    // usando metodo de impl 2
    square1.print_something();

    println!("El cuadrado tiene un area de: {}",square1.area());
    /*
     *  Como el método area() se ha extendido al struct Rectangle, podemos hacer uso de este para
     *  poder crear un método que compartiran todas las instancias del struct Rectangle.
     * */
    println!("El area del rectangulo es: {}",rect1.area());
    rect1.width();

    println!("Can rect1 hold rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect1 hold rect3: {}", rect1.can_hold(&rect3));
}

