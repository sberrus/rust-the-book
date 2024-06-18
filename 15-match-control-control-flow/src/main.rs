/*
 * El flujo de control match nos permite comparar valores y ejecutar expresiones en base si un
 * valor coincide con un patron.
 *
 * Los patrones pueden ser variablesm wildcards, string literales entre otras cosas.
 *
 * Para ver este comportamiento haremos una estructura lógica para una máquina que separa monedas.
 *
 *
 * */

// tipos de moneda
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Hello, world!");
}

// devuelve el valor de la moneda en centimos
fn value_in_cents (coin: Coin) -> u8 {

    // usando la keyword match iniciamos el cuerpo del match para realizar las evaluaciones
    // La estructura es parecido a un if, con la diferencia de que en el caso de los if, el valor a
    // evaluar debe ser un booleano. En el caso de los match, pueden ser cualquier valor. Siempre
    // que coincida el patron que proporcionamos, devolvera el valor que le indiquemos despues de
    // =>.
    // Cada una de los patrones con su respectivo valor a devolver, se le llama `arm` o pata.
    match coin {
        // Si la expresión es corta, no hace falta abrir y cerrar llaves para la expresión.
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Se puede, en el caso de que sea una expresión más compleja, abrir y cerrar llaves.
        Coin::Quarter => {
            println!("Lucky quarter!");
            25
        },
    }
}
