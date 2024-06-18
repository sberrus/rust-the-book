/**
 *  Option<T> y el manejo de los valores de tipo None
 *
 *  En rust, el valor null no tiene una forma directa de usarlo, esto debido a que con null, se han
 *  tenido muchos problemas con otros lenguajes de programación y es propenso a tener bugs, para
 *  evitar este problema con el manejo de los null, tenemos un tipo especial de enum que ya viene
 *  predeterminadamente en la libreria estandard llamado Option<T>.
 *
 *  Este enum tiene 2 campos None y Some() los cuales nos ayudan a capturar un valor que
 *  posiblemente sea vacío y rust nos obliga a que manejemos implicitamente el valor que captura
 *  este enum.
 *
 *  De esta forma podemos manejar explícitamente los valores que puedan contener datos vacíos y
 *  manjear los casos desde compilación.
 *
 *  La <T> es un génerico, pero si quieres hacer un i32 + Option<i32> el compilador va a devolver
 *  error porque no son el mismo tipo de dato.
 * */

fn main() {

    // definiendo datos de tipo Some
    let some_number = Some(5);
    let some_char = Some('e');

    // capturando de manera segura un valor de tipo None
    let absent_number:Option<i32> = None;

    // let sum = some_number + 32; -> esto devuelve error
    // Para poder usar el valor que contiene some_number, debemos convertirlo a number primero
    // Para obtener el valor de un Some, hay muchas estrategias pero siempre se debe manejar el
    // caso cuando el valor es None
    println!("Hello, world!");
}
