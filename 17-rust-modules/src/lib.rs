// AGRPUANDO MÓDULOS EN RUST
/**
En rust tenemos una forma de ordenar el código por módulos, esto nos permite organizar más eficientemente los bloques de código que sean relacionados con una funcionalidad mejorando la redabilidad del mismo.

En esta sección veremos las ventajas de esta caracteristica con un ejemplo práctico.

De forma predeterminada, los módulos son privados, por lo que si queremos dejar que se pueda acceder a ciertas partes del código debemos indicarlos explicitamente en el código.

EN este ejemplo, haremos un restaurante, de forma que podamos visualizar claramente las caracteristicas de los módulos.
*/

// Esta es la sección pública del restaurant, como en la vida real, esta es la zona que cualquiera puede acceder e interactauar con el mismo.
/*
Otra cosa es que tenemos un módulo que contiene módulos internos. Esto nos permite dejar de forma más clara y legible como van a estar estructurados las piezas dentro del código.
*/

/*
Tanto las librerias como los binarios, forman parte de un crate superior directamente llamado crate, siendo este la raiz de la estructura de arbol de los crates de un programa/libreria de rust. Para visualizarlo mejor podemos verlo de la siguietne forma:

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

De esta forma también podemos ver ciertos aspectos como que take_order es primo de add_to_waitlist porque ambos estan dentro del abuelo front_of_house.

Esto ayuda enormemente a la hora de leer el código y entender más claramente como va a estar estructurado la arquitectura del mismo. A su vez, podemos ver claramente que módulos son parte de cuales otros.
*/

// PRIVACIDAD DE LOS MóDULOS DE RUST Y LOS PATHS
/*
En rust, otro de los conceptos que debemos tener en cuenta es los PATHS siendo uno de los conceptos para entender correctamente como rust maneja la privacidad y el scope del código.ç

Veremos más en detalle conceptos como `use` `pub` `as` entre otras cosas.
*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}

/*
    LOS PATHS

    Al igual que en los filesystems, hay dos tipso de formas para acceder a los crates dentro del proyecto.

    - Absolute: Los paths absolutes son los paths que empiezan desde el crate root siendo este la raiz del programa. En el caso de los crates externos, se accede a estos usando el nombre del crate como raiz; y en el caso del crate actual, hacemos uso de "literal crate" haciendo referencia al crate en sí mismo donde se encuentra.

    - Relative: Estos empiezan desde el módulo actual y usa self y super como identificadores.

    En ambos casos van seguidos de identificadores separados por (::).

    Ahora supongamos que queremos acceder a la función add_to_waitlist() desde el cuerpo de la librearia como lo hacemos?
*/

pub fn eat_at_restaurant() {
    // Absolute Path
    crate::front_of_house::hosting::add_to_waitlist(); // En este caso, la kw crate, hace referencia al root crate, siendo este el fichero lib.rs o main.rs

    // Relative path
    front_of_house::hosting::add_to_waitlist() // En este caso, busca se busca el módulo dentro del contexto.

    /*
       Otro tema a tener en cuenta es el hecho del scope. En los módulos y las funciones, tenemos definidas con la kw pub al principio, si no definimos el scope de la función, esta no permitira acceder al contenido de la misma. Esto añade una capa extra de seguridad y de legibilidad permitiendo saber a que partes del código podemos acceder y consumir.

       Por defecto el scope de los módulos y las funciones es private, por lo que si queremos que esa pieza sea alcanzada desde cualqueir parte del código.
    */
}

// USANDO PATHS RELATIVES USANDO LA KW super

/*
    La kw super nos permite acceder a la ruta relativa del módulo padre de donde se esta usando el super, sería el equivalenta a usar (..) en los filesystems. Por lo que super referencia al módulo padre del módulo que lo invoca.

    para visualizarlo, vamos a crear el back_of_house haciendo uso del super para poder acceder a una función que se encuentra en el parent del módulo
*/

fn deliver_order() {} // esta dentro del módulo padre

// Módulo hijo
mod back_of_house {
    // Función interna del módulo hijo
    fn fix_incorrect_order() {
        // Accede a la función hermana dentro de back_of_house()
        cook_order();
        // Accedemos a la función deliver_order que se encuentra en el módulo padre
        super::deliver_order();
    }

    // función interna del módulo hijo back_of_house
    fn cook_order() {}
}
