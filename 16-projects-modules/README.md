# MóDULOS EN RUST

En cualquier proyecto, llega un momento en el que el código empieza a crecer de tal manera quetener todo en un solo fichero, a nivel de organización es múy complicado. Para manejar esto,muchos lengaujes de programación tienne el concepto de módulos, los cuales nos permitenorganizar el codigo en chunks más pequeños y mantenibles para hacer que el código se máslegible y escalable. Rust no es la excepción por lo que veremos como se crean e ióportanmodulos en este lenguaje de programación.

Cargo además ofrece una funcionalidad llamada workspaces que nos permiten generar y ordenar el código de manera más eficiente.

Para englobar estos conceptos necesitamos tener presente lo siguiente:

- Packages: permite tener un entorno el cual construir, testear y compartir crates.
- Crates: Un arbol de módulos que permiten crear una libreria o un ejecutable
- Modules and use: Permite controlar la organización, el scope, y la privacidad de los paths
- Paths: es una forma de llamar a un elemento dentro de nuestro código ya sea un struct, función o un módulo.
  
## CRATES

Los crates son la unidad más pequeña de código que el compilador de rust considera.

Los crates pueden ser de 2 tipos, binary o library. Los binary son los crates que compilan un binario que puedes ejecutar. Lo principal que podemos observar es que los binarios cuentan con al función main indicando esta al compilador que es un binario ejecutable y que esa función es el punto de partida de todo el binario.

Las librerias no cuentan con la función main y no pueden ser compiladas a un ejecutable. Por otro lado son útiles porque permiten crear funcionalidades que pueden ser compartidas entre proyectos.

Las librerias deben contnener un fichero raiz el cual define cual es el punto de acceso a las funcionalidades de la libreria. Esto es útil tambien a la hora de definir la privacidad y el scope de los paths dentro de la misma.

## PACKAGES

Los packages son un grupo de uno o varios crates que permiten exportar funcionalidades. Este contiene un Cargo.toml el cual define como se debe construir el package en cuestión.

Podemos generara un package facilmente y lo hemos hecho con el uso del comando `cargo new <nombre_proyecto>`

## MÓDULOS

Veremos a continuación como es el proceso que el compilador realiza a la hora de definir como lee y compila el código fuente de un proyecto realizado en rust, en este caso usaremos como ejemplo el módulo `garden`, para poder ejemplificarlo mejor.

1. Busca el fichero raiz: En el caso de que sea una librería busca el `lib.rs` en el caso de un binario, busca el fichero `main.rs`.
1. Se declarar el nombre del fichero con la kw `mod`. Por lo que si en una libraria queremos definir un nombre para el módulo, hacemos uso de la kw `mod garden;`. Luego el compilador va a buscar el codigo fuente en los siguientes lugares:

- Inline: en el fichero raiz buscando en la primera línea {mod garden}.
- En el fichero `src/garden.rs`.
- En el fichero `src/garden/mod.rs`.

1. Busca sóbmodulos. Con la kw `mod`, por ejemplo, podemos hacer uso del sóbmodulo `vegetables` de la siguiente manera:

- Inline: en el fichero raiz buscando en la primera línea {mod vegetables}.
- En el fichero `src/garden/vegetables.rs`.
- En el fichero `src/garden/vegetables/mod.rs`.

1. Busca los Paths. Una vez definidos los módulos, podemos acceder a ellos desde cualquier punto dentro de nuestro crate. Por ejemplo, si queremos acceder al type `Asparagus` dentro del módulo vegetables, debemos acceder mediante `crate::garden::vegetables::Asparagus`.

1. Busca los Paths públicos y privados. El código dentro de un módulo es privado el acceso desde el padre de manera predeterminada. Para que un módulo sea público para el padre, debemos definir el módulo como `pub mod` en lugar de `mod` a secas. Para que ciertas piezas del código sean publicas, también debemos hacer uso de la kw `pub` a la hora de definir la pieza que quremos descubrir.

1. Busca los kw `use`. Para evitar el estar repitiendo constantemente el Path completo de una pieza de código como `crate::garden::vegetables::Asparagus` cada vez que querramos acceder a dicha pieza, hacemos uso de la kw `use crate::garden::vegetables::Asparagus`; la cual nos permite en vez de usar todo el Path, nos permite acceder a la misma mediante el namespace `Asparagus` dentro del código donde queramos acceder al mismo.

Para ilustrar lo anterior, crearemos un crate `backyard` el cual tendrá tood lo definido anteriormente.

``` text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```
