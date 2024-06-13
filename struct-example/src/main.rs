/*
 * Crearemos una función que calcule el area de un rectangulo
 * */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main (){
    // definimos un rectangulo usando el struct Rectangle, esto nos ayuda a mejorar la legibilidad
    // del coódigo y a mejorar la estructura del mismo.
    // Si bien es cierto que podemos hacer lo mismo pasando los datos directamente o mediante una
    // tupla, para mejorar principalmente la legibilidad dle código, usamos strcuts en este caso.
    let rect1 = Rectangle {
        height: dbg!(30*2),
        width:50,
    };

    // Viendo en modo debug el contenido de rect1
    // Por defecto, dentro de los corchetes solo podemos pasar datos primitivos de manera
    // predeterminada en rust que contengan el trait std::fmt::Display. Esto con el fin de que solo
    // podamos mostrar datos válidos y que no tengamos bugs inesperados.
    //
    // En el caso de que queramos ver un dato que no contenga el trait std::fmt::Display, hacemos
    // uso dentro de los corchetes `:?`, de esta forma vemos datos de debug de la varuable pasada y
    // le pasamos al struct la funcionalidad mediante `#[derive(Debug)`]. De esta forma podemos
    // extender las funcionalidades de Debug al struct Rectangle para que pueda mostrar datos de
    // debug en la consola explicitamente.
    //
    // Hay ciertos datos no primitivos que ya contienen las funcionalidades de debug
    // predeterminadamente, y hay otras como los structs que explicitamente tenemos que
    // extenderselas.
    println!("Viendo rect1 en modo debug con {{:?}} -> {:?}",rect1);

    // Usando {:#?} podemos ver el dato en multiples líneas tal cual aparece en el código.
    println!("Viendo rect1 en modo debug con {{:#?}} -> {:#?}",rect1);

    // Otra forma que tenemos de debugear una variable es con el uso del macro dbg!(), la
    // diferencia es que dbg!() devuelve un stderr y en el caso de println!() devuelve un stdout
    // Lo bueno de dbg!() es que muestra otra información como la línea en la que se encuentra el
    // debug
    dbg!(&rect1);

    // pitando el area del rectangulo
    println!("El area del rectangulo es {}",area(&rect1));

}

// Declarando la función de esta forma, queda mucho más claro que queremos pasarle esta struct que
// sabemos que tiene esos campos. Si esto lo acompañamos de buenos comentarios, queda muy claro que
// se quiere hacer y que datos queremos pasarle a esta función. Aumentando la legibilidad del
// código.
fn area (rectangle:&Rectangle)-> u32{
    rectangle.width * rectangle.height
}
