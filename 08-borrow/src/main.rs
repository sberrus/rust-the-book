fn main() {
    let mut s = String::from("Hola");

    // haciendo uso de los & estamos indicandole a rust que debemos pedir prestado el valor de s
    // con su información del stack y alocación para que sea usado en otras partes del código.
    // Funciona como una especie de puntero a la variable s pero con unas reglas diferentes a otros
    // lenguajes como podían ser C que tienen un control más preciso de la memoria.
    let m1 = &s;
    let m2 = &s;

    println!("{} {}",m1,m2);
    println!("{} {}",m1,m2);

    // let m4 = &mut s; -> Fallará porque no se pueden tener dos referencias mutables dentro del
    // mismo contexto. Esto se hace para evitar carreras de asignación de datos y para que el
    // software no tenga comportamientos anomalos.
    let m3 = &mut s;

    // println!("{m1}{m2}"); -> el compilador no lo permite porque justo antes tenemos definida una
    // variable que pide prestada de forma mutable la referencia de las variables que estamos
    // intenado imprimir.
   
    // actualizamos s a traves de m3
    m3.push_str(" mundo");
    println!("{}",m3);
    // Lo que se debe tener en consideración es que no se pueden crear 2 variables mutables de la
    // misma referencia juntas. Deben crearse y luego utilizarse
    m3.push_str(" prest...");
    let m4 = &mut s;
    m4.push_str("cosa...");
    println!("{}",m4);

    // pasamos s a la función para que mutemos s a través de la misma
    let s2 = retrieve_mutated_value(&mut s);
    println!("{}",s2);

    // si accedemos a s directamente tenemos todos los cambios que se hicieron en m3 y en la
    // función retrieve_mutated_value() ya que en esta se encontraban los cambios
    println!("{}",s);
}

fn retrieve_mutated_value(si:&mut String) -> &String{
    si.push_str(" another one");
    si
}


