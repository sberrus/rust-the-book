// TIPOS DE DATOS EN RUST
fn main() {
    /*
     Los números como en alugnos lenguajes fuertemente tipados, se usa i para para los números con signo y u para los números
     exclusivamente positivos positivos. Ya despues se le asigna el tamaño de bits del espacio en memoria.
    */

    let _solo_positivo: u8 = 15; // positivo
    let _positivo_negativo: i8 = -15; // positivo y negativo
    let _flotante: f64 = 3.5235; // tipo punto flotante, por defecto 64 bits
    
    
    // Tabla de tipos de números con sus respectivos espacios en memoria
    /*
     * Length   Signed  Unsigned
     * 8-bit    i8  u8
     * 16-bit   i16 u16
     * 32-bit   i32 u32
     * 64-bit   i64 u64
     * 128-bit  i128    u128
     * arch isize   usize
     */
    // En el caso de los isize y los usize depende principalmente de la arquitectura del sistema.


    // booleanos
    let _t = false;
    let _x: bool = true;

    // números de punto flotante
    /*
     * La diferencia entre estos radica en la precisión, siendo los f64 más precisos debido a que
     * estos tienen la capacidad de almacenar muchos más valores decimales que f32, pero en contra
     * parte, f32 es mucho menos intensivo en consumo de memoria y es más rápido. f32 suele ser
     * suficiente para la mayoria de casos, pero en el caso de realizar tareas más intensivas de
     * precisión como pueda ser calculo analítico o financiero, es preferible usar un tipo de dato
     * mucho más preciso para evitar errores sobretodo en el redondeo de los mismos.
     */

    let _f = 2.0; // predeterminado :f64
    let _f2: f32 = 5.2;


    //OPERADORES ARITMÉTICOS

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;
    
    // TIPO CHAR
    /*
     * Los char a diferencia de los literal strings a la hora de definirlos se realiza usando '' en
     * vez de "". Los datos tipo char son datos de 4 bytes para almacenar una unica letra o
     * caracter dentro de los valores Unicode Scalar
     */

    let _c = 'z';
    let _z: char ='Z';
    let _emoji = '😊';

    // println!("El caracter es '{}'",_c);

    // TIPOS COMPUESTOS tuples
    // Los valores compuestos o tuplas son una estructura de datos que permite agrupar varios datos
    // dentro de un mismo espacio de memoria siendo estos de distintos tipo si se desea.
    //
    // Para definir una tupla, al momento de definir el tipo de dato, arropamos entre paréntesis
    // los tipso de datos separados por comas (datatype,datatype,datatype) y asignamos el valor de
    // la misma manera, pero respetando el tipo de dato que hemos definido en la estructura de la
    // tupla.

    let tup: (i32,f64,u8)=(500,6.5,1);

    // Para acceder a un elemento de la tupla podemos realizar una destructuración para acceder al
    // valor que deseemos.
    
    let (tupa,tupb,tupc) = tup;

    println!("Valor de tup en la posición 0: {tupa}");
    println!("Valor de tup en la posición 1: {tupb}");
    println!("Valor de tup en la posición 2: {tupc}");

    // La otra forma que tenemos para acceder a un valor es mediante dot notation 
    println!("Valor obtenido con dot notation en posición 0: {}",tup.0);
    println!("Valor obtenido con dot notation en posición 1: {}",tup.1);
    println!("Valor obtenido con dot notation en posición 2: {}",tup.2);

    // Las tuplas que no tienen ningún valor asignado se les llama unit

    // let _unit:(u32,u32,u32) = ();

    // ARRAY
    // arrays de toda la vida
    //
    // En rust tenemos varios tipos de datos para almacenar datos secuencialmente apuntando al
    // mismo espacio en memoria. Ya hemos visto las tuplas estas nos permiten almacenar
    // secuencialmente valores permitiendonos que sean de distinto tipo cada valor. En el caso de
    // los arrays tenemos dos "arrays" o estructuras parecidas; los arrays y los vector.
    //
    // La principal diferencia entre los array y vectores es que los arrays tienen un espacio
    // definido de memoria, por lo que debemos asignar cuantos valores podrá tener este array. En
    // el caso de los vectores, estos pueden crecer y decrecer bajo demanda.
    //
    // Ambos, tanto array como vectores necesitan definir un tipo de dato y este solo podrá
    // almacenar este tipo de dato.

    // declarar un array manera común
    let arr = [1,2,4,5];

    // declarar un array con 5 valores de mismo tipo
    let _arr2:[i32;5] = [1,1,2,2,1];

    // declarar un array que contendrá un valor x repetido una cantidad y de veces.
    let _arr3 = [3,5]; // Este array contendra [3,3,3,3,3]

    // accediendo a valor de array en posición x
    println!("Accediendo a valor de array en posición 0: {}", arr[0]);

    //
    //
    //
    // crear app para obtener un valor de un array dependiendo de su posición pasada por input

    use std::io;
   
    // array with values
    let a = [1,2,3,4,5];

    // index store
    let mut index = String::new();

    // print question
    println!("\nType a number to find an element in array {:?}: ",a);

    // read index from input
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    println!();
    // parse index from string to number
    let index: usize = index.trim().parse().expect("Index enternet was not a number");
    
    // find element given an index
    let element = a[index];

    println!("The value of the element at index: {index} is: {element}");
}
