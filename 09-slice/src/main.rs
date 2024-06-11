/*
 * SLICES 
 * 
 * Los slices son una especie de referencia sin que esta tenga dueño
 */


fn main() {
    // Creamos la var word para almacenar el string mutable
    let mut word = String::from("Hello world");
    
    // valor que vamos a usar en el slice
    let word2 = String::from("Hello world");
    let word3 = String::from("Helloworld");

    // obtenemos la posición del primer espacio, de lo contrario, devuelve el tamaño de todo el
    // texto.
    let len = first_space_index(&word);

    // imprimimos el valor obtenido de la función
    println!("El valor len es: {}",len);

    // limpiamos la variable string
    word.clear();

    // Volvemos a obtener el valor
    println!("Valor word limpiado!");
    println!("El valor len es: {}",len);

    // usando slices
    // Para usar los slices usamos los corchetes indicando, separado por (..) el indice inicial y
    // el indice final más uno del array que estamos evaluando.
    //
    // Al ser una referencia, debemos usar &<variable a referenciar>[slice]
    let hello = &word2[0..5]; // valores desde posición 0 a posición 4 [0..5]
                              // En este caso, también podemos usar [..5] el cual indica desde el
                              // index 0 hasta el 4 respectivamente.
    let world = &word2[6..11]; // valores desde posición 6 a posición 10 [6..11]
    let word2_len = word2.len();
    let whole_world = &word2[..word2_len]; // podemos pasar una variable si necesitamos que ese
                                           // indice se calcule previamente o si queremos
                                           // almacenarlo en otro sitio
    let whole_world2 = &word2[..]; // pasando (..) sin indices, podemos obtener todo el valor de la
                                   // variable.

    println!("valor hello: {}",hello);
    println!("valor world: {}",world);
    println!("{}",word2.len());
    println!("valor whole_world: {}",whole_world);

    // obteniendo valor como slice
    let word_found = first_word(&word3);

    println!("{}",word_found);
    println!("{}",first_word_literal_string("También podemos pasar un string literal ya que este valor es similar a un string slice"));
}

// Creamos una función que recibe un &String como argumento y definimos que devuelva un usize.
fn first_space_index(s:&String) -> usize {
    let bytes = s.as_bytes(); // convertimos el string a un array de bytes con el método as_bytes()
    let iter_values = bytes.iter(); // devuelve un iterable de un array
    let iter_values_enumerate = bytes.iter().enumerate(); // arropa al iterador y devuelve por cada
                                                          // valor, una tupla con un par
                                                          // (index,&valor)
    println!("iter_values contiene: {:?}",iter_values);
    println!("iter_values_enumerate contiene: {:?}",iter_values_enumerate);

    // prueba
    for value in iter_values_enumerate{
        println!("{:?}",value);
    }

    // b' ' devuelve le byte literal del caracter que pases dentro de las comillas.
    println!("{}",b' ');


    // Se iteran todos los elementos el array
    // Hay que ver que poniendo &item esta desreferenciando el valor de item
    // Lo que pasa es que cuando intentas hacer un equal (==) si estas evaluando un valor y una
    // referencia, estos al hacer (valor == referencia) va a dar false, ya que el valor no se
    // encuentra en la referencia de la misma manera a la hora de hacer la comparación.
    for (i,&item) in bytes.iter().enumerate() { // bytes.iter() devuelve un iterable 

        // evaluamos si el valor item es igual a espacio
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s:&String) -> &str {
    let bytes = s.as_bytes(); // convertimso el string a array de bytes

    // recorremos todos los elementos
    for (i, &item) in bytes.iter().enumerate() {
        // evaluamos si el byte es igual al byte b' '
        if item == b' ' {
            // si es true, devolvemos el slice empezando desde el indice 0 hasta el indice donde se
            // encuentra el primer byte b' ' 
            return &s[..i];
        }
    }

    // si no hay bytes b' ' se devuelve todo el string
    &s[..]
}

// De esta forma es mucho más flexible ya que podemos pasar un string literal, un slice o una
// referencia de un String ya que en la firma &str nos da esa flexibilidad.
fn first_word_literal_string(s:&str) -> &str {
    let bytes = s.as_bytes(); // convertimso el string a array de bytes

    // recorremos todos los elementos
    for (i, &item) in bytes.iter().enumerate() {
        // evaluamos si el byte es igual al byte b' '
        if item == b' ' {
            // si es true, devolvemos el slice empezando desde el indice 0 hasta el indice donde se
            // encuentra el primer byte b' ' 
            return &s[..i];
        }
    }

    // si no hay bytes b' ' se devuelve todo el string
    &s[..]
}
