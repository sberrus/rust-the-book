/*
    USANDO LA KW `use` PARA MEJORAR LA LEGIBILIDAD DEL CÓDIGO

    Hay veces en la que, a mayor la complejidad del a estructura del código, podemos encontrarnos con módulos que para importarlos tenemos que llamar a todo el churro hasta llegar al módulo que quieres usar.

    Para evitar esto, rust da la kw `use` que nos permite definir la ruta al módulo una sola vez, y ya dentro del código, solo trabajaremos con el namespace del módulo en especifico para que sea más sencillo trabajar y acceder a este.

    Cabe destacar un aspecto importante qué es que cuando hacemos uso de la kw `use` estamos creando una ruta que solo afecta al scope donde es definido dicha kw, por lo que debemos tener presente esto a la hora de definir cómo y cuando usarla.
*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // -> Si lo declaramos aquí, solo va a poder ser accedido directamente en este scope (crate root), si queremos utilizarlo, por ejemplo, dentro del módulo costumer, este no va a ser capaz de alcanzarlo.

/*
    IDIOMATIC `use` PATHS

    En el caso anterior, si es cierto que podemos llamar a la función directamente con la siguiente declaración: `use crate::front_of_house::hosting::add_to_waitlist;`, esto también haría que cada vez que queramos usar otras funciones, debamos importarlas una a una. Si importamos el módulo, siempre que lo permitan las firmas de privacidad, podremos acceder a las funciones que haya disponibles dentro del modulo llamando directamente a este.

    Es realmente útil como en el siguiente caso, en el cual queremos traer de la librería estandar el módulo HashMap para hacer uso de sus funcionalidades.
*/

use std::collections::HashMap; // -> De esta forma podemos hacer uso de las fucnionalidades del módulo, y además, si este crece, podremos acceder a todas sus funcionalidades.

/*
    USO DE KW `as`
    Queremos traer 2 módulos que tienen el mismo namespace `Result`. Si los importamos tal cual, veremos que el compilador va a decir que hay un conflicto ya que Result esta siendo definido múltiples veces en el mismo scope. Por lo que si queremos hacer uso de ambos módulos, rust nos ofrece una kw `as` que nos permite renombrar un namespace para evitar estos errores
*/
use std::fmt::Result;
use std::io::Result as IoResult; // -> Ahora podremos acceder al módulo Result del módulo io a traves del namespace IoResult y evitamos el conflicto anterior.

mod costumer {
    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist(); -> si intentamos acceder al modulo hosting desde el modulo costumer directamente, nos va a devolver un error ya que no va a ser capaz de llegar a este. Si queremos poder acceder a este módulo podríamos hacer uso de `super::` para acceder primero al módulo padre, en este caso el crate per sé, y ahí si podemos acceder al namespace `super::hosting::<resto_de_la_ruta>`
        super::hosting::add_to_waitlist(); // -> válido

        // creando un map con haciendo uso del módulo HasMap
        let mut map: super::HashMap<i32, i32> = super::HashMap::new();
        map.insert(255, 2);
    }
}

/*
    Exportando módulos importados con `use` usando en conjunto `pub` y `use`. re-exporting

    Existe la posibilidad de exportar namespaces de módulos que han sido sido importados previamente a otro módulo usando en conjunto loas kw `use` y `pub` como en el siguiente ejemplo
*/

// En este punto estamos definiendo en el módulo exa una importación del módulo Result de io. Al hacerlo público, hacemos posible que desde el módulo exa, podamos exportar y usar como un módulo interno de exa el módulo Result de io.
pub mod exa {
    pub use std::io::Result; // exportamos publico el módulo Result en exa
}

use exa::Result as ExaResult; // usamos el módulo Result de io a traves de exa y además, renombramos el namespace con `as`.
