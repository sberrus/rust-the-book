// De esta forma importamos librerias para poder hacer uso de la libreria io para poder pedir
// inputs a los usuarios.
// En este caso estamos importando desde la libreria standar (std) la libreria (io).

// La libreria estandar (std) es una libreria que esta presente en todos los programas de 
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // usando la libreria rand para generar un número aleatorio.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop{

        std::process::Command::new("clear").status().unwrap();
        println!("Please input your guess.");

        let mut guess = String::new();

        // Hacemos uso de read_line para capturar el input del usuario. Como podemos ver, tenemos el
        // método expect() el cual nos ayuda a evaluar si se ha podido capturar correctamente el input
        // del usuario.
        // Esto nos ayuda para poder enviar mensajes de error de manera más declarativa y mejorar el
        // debug.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Desglozando hacemos lo siguiente: Primero a la variable guess podemos aprovechar una
        // propiedad de rust llamada shadow la cual nos permite asignar en una variable el resultado de
        // la misma variable tratada sin tener que crear otra variable. Por lo que a la variable guess,
        // sin tener que crear otra variable, cambiamos el tipo de dato a u32.
        //
        // Luego con la propiedad trim() eliminamos los espacios en blanco y los saltos de línea y con
        // parse() lo que hacemos es parsear el dato que tenga almacenado guess en un principio y lo
        // cambia al tipo de dato que hemos asignado a la variable ne cuestión. En este caso parseamos
        // de string a u32. Y para finalizar expect("<mensaje>") nos permite, en caso de que haya algún
        // error, devolver un mensaje de error para debuggear.
        // let guess: u32 = guess.trim().parse().expect("Plese type a numbers");

        // Podemos cambiar este fragmento de código para poder manejar los tipos de datos que nos
        // pasan. Para hacer esto vamos a hacer uso de la expresión match para evaluar el tipo de
        // dato.
        let guess: u32 = match guess.trim().parse() {
            // En el caso de que el valor satisfaga el pattern Ok(num) va a ejevutar el código. De
            // lo contrarío devolvería Err(_) y continue para que continue con el loop sin que
            // genere el panik
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {guess}");

        // De esta forma podemos pasar variables a el print. Podemos hacerlo pasando una variable
        // directamente dentro de {} o podemos hacer el uso de la evaluación de expresión poniendo {}
        // si nada dentro y pasandole valores que va a remplazar dentro de cada {} en orden.
        //    let mensaje_principal:&str = "El valor capturado es:";
        //  println!("{mensaje_principal} {guess} {}","maravilloso!")

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                //Usamos este break dentro de el cuerpo de Equal hacer que el programa salga del
                //bucle y finalize.
                break;
            },
        }

        let mut temp = String::new();

        println!("Press Enter to continue...");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

    }
}
