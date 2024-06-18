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

    let _five = Some(5);
    let _six = plus_one(five); // -> Some(6)
    let _none = plus_one(None); // -> None

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // catch-all variable es la variable que en el caso de que ningún patrón coincida y solo
        // quieres manejar ciertos casos. La última pata es la catch-all la cual puede tener o una
        // variable o un _. 
        //
        // La variable permite capturar el valor que no tenga un patrón y nos permite usarlo dentro
        // del cuerpo del arm.
        //
        // En el caso de _, se usa cuando queremos tener el catch-all, pero no queremos hacer nada
        // con el valor que no contenga patrón.
        other => move_player(other),
        // _ => reroll(), -> no hacemos nada con el valor, pero ejecutamos la función reroll()
        // En el caso de que no queramos hacer nada en el caso de que ningún patrçon coincida
        // simplemente hacemos un...
        // _ => (), -> de esta forma, en el caso de que ningún patrón coincida, el código seguirá
        // realizando las ejecuciones siguientes.
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // si recive un None, devuelve None
        // Es importante recalcar que siempre se debe manejar el caso de None en las options, esto
        // para evitar que en el caso de que recibamos algún None, nos ocasione un bug. Ya si hay
        // algún bug es porque no se manejo correctamente el caso de None.
        None => None,
        // si recive Some(T) con hace T + 1, i captura el valor que se le pasa a Some(i) por lo que
        // de esta forma podemos trabajar con el valor de i
        Some(i) => Some(i+1),
    }
}

// funciones para el dice_roll
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
