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

// PRIVACIDAD DE LOS MODULOS DE RUST Y LOS PATHS
/*
En rust, otro de los conceptos que debemos tener en cuenta es los PATHS siendo uno de los conceptos para entender correctamente como rust maneja la privacidad y el scope del código.ç

Veremos más en detalle conceptos como `use` `pub` `as` entre otras cosas.
*/


mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
