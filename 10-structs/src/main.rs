/*
 *  Structs
 *
 *  Los structs es una estructura que es similar a las clases en lenguajes como javascript o java.
 *
 *  Además permite declarar de una forma más semantica, el tipo de dato que va a contener y su
 *  estructura interna.
 * */
/*
 *  Tuple structs
 *
 *  Son tuples que tienen el mismo comportamiento que los structs. Esto es útil a la hora de poder
 *  crear tuplas con nombres más semánticos 
 * */

// Definimos un struct con la kw `struct` y definimos sus campoes y sus tipos de datos
// separados por comas.
struct User {
    active: bool,
    username:String,
    email:String,
    sign_in_count: u64,
}

// definición de struc tuples
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

// definiendo structs unit-like
// estos son útiles para cuando queremos crear un struc que luego extenderemos con los Traits para
// extender funcionalidades.
struct AllwaysEqual;

fn main (){
    // usando los tuple struct
    let black = Color(0,0,0);

    // Declaramos una variable con el tipo de dato heredado del struc de la siguiente manera
    let user1 = User {
        active: true,
        username: String::from("usuario1"),
        email: String::from("usuario1@somemail.com"),
        sign_in_count: 1,
    };

    // Podemos acceder a los datos usando dotnotation
    println!("-----");
    println!("Usuario: {}",user1.username);
    println!("Correo: {}",user1.email);
    println!("-----");

    // Podemos definir el struct para que sea mutable
    // Importante destacar que toda la estructura va a ser mutable, no podemos hacer que solo una
    // campo sea mutable.
    let mut user2 = User {
        active: true,
        username: String::from("usuario2"),
        email: String::from("usuario2@somemail.com"),
        sign_in_count: 1,
    };

    // consumimos valor inicial
    println!("Usuario: {}", user2.username);

    // modificamos el campo
    user2.username = String::from("usuario2 actualizado");

    // consumimos valor actualizado
    println!("Usuario: {}", user2.username);

    // capturamos el struct construido desde la función
    let usuario_contruido = build_user(String::from("usuario construido"),String::from("usuario_contruido@someemail.com"));

    // consumimos el struct construido
    println!("Usuario: {}",usuario_contruido.username);

    // construimos un usuario a partir de otro usuario
    let user_copy = generate_user_from_another_user(usuario_contruido,String::from("usuario copiado"));

    println!("Usuario construido a partir de otro: {}",user_copy.username);
}

// podemos devolver el struct en una función, en la firma podemos definir que sea el struct el
// valor que retorna la misma
fn build_user(username:String, email:String) -> User {

// podemos usar la siguiente forma para definir un campo
//    User {
//        active: true,
//        username: username,
//        email: email,
//        sign_in_count: 1,
//    }

    // Si el nombre de la campo coincide con el nombre de la variabla de la firma, podemos
    // indicar directamente el namespace en la campo del struct y el valor se pasará
    // directamente.
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


// Podemos usar los datos de otro User para rellenar los datos de un nuevo user
fn generate_user_from_another_user(user:User, username:String) -> User {

    // Para evitar tener que escribir todos los campos de un struct uno a uno, en el caso de que
    // tengamos un struct que sea muy largo, podemos hacer uso del doble dot syntax para copiar
    // todos los campos de un struct en otro struct
    //    User {
    //        active: user.active,
    //        username,
    //        email:user.email,
    //        sign_in_count: user.sign_in_count,
    //    }

    // copiando todos los campos de un struct y modificando solo los que deseamos modificar con
    // doble dot
    
    User {
        username,
        ..user
    }
}


