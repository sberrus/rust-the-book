// En el contexto de las funciones, la función main es la más importante ya que esta es el punto de
// inciio de nuestro programa. 
fn main() {
    println!("Hello, world!");

    // invocando función
    another_hello();

    // invocando función y pasando argumentos
    print_age(25);

    // probando
    let text: String =  long_message();
    println!("{}",text);
    tuple_func((20,"Jaimito"));

    // expresiones vs statements
    statement_func();
    let hortaliza: String = expression_func();
    println!("La {} es una hortaliza espectacular!",hortaliza);

    // expressiones que no son funciones
    expression_to_var();
}

// DECLARANDO Y CONSUMIENDO TU PRIMERA FUNCIÓN

// para declarar una función usamos la kw fn seguido del identificador, los parentesis para los
// parámetros y luego el cuerpo de la función.

// declarando función
fn another_hello (){
    println!("Hello from function!");
}

// PARÁMETROS DE UNA FUNCIÓN

// como en el resto de lenguajes de programación de tipado estático, para definir una función con
// parámetros, debemos dentro del cuerpo de los parámetros en la firma de la función, debemos
// definir las variables que van a alojar esos parámetros para luego usarlos en el cuerpo de la
// función. En el caso de rust, debemos también asignar el tipo de dato que va a almacenar dicho
// parámetro.

fn print_age(age:i32){
    println!("You are {} years old",age);
}

// probando un tema aquí :v
fn long_message()->String{

    let mut l_string = String::from("");

    l_string = l_string + "este ";
    l_string = l_string + "texto ";
    l_string = l_string + "es mu largo";

    return l_string;
}

// función con tupla
fn tuple_func(tup:(i32,&str)){
    let (age,name)= tup;
    println!("{} is {} old",name,age);
}

// DIFERENCIA ENTRE DECLARACION (STATEMENT) Y EXPRESION (EXPRESSION)
//
// La principal diferencia entre ambos tipso de funciones es que las declaraciones no devuelven
// nada. Solo ejecutan una serie de instrucciones sin devolver ningún valor. 
//
// Por otra parte, las expresiones son funciones que evaluan la devolución de un valor. Son las que
// si realizan operaciones para devolver un valor. Esto es importante a la hora de definir ciertos
// aspectos del lenguajes.

fn statement_func(){
    println!("Solo sirvo para imprimir este texto");
}

fn expression_func() -> String{
    println!("En este caso, además de imprimir este texto, devuelvo el valor 'patata'");
    "patata".to_string()
}

// Las expresiones no tienen que ser necesariamente funciones, podemos definir una expresión para
// evaluar un valor que sea usado en una variable

fn expression_to_var(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("El valor de y es: {}", y)
}
