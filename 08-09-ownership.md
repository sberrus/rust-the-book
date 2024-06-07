# Entendiendo el ownership

Este es un concepto de como rust maneja el scope de las variables y como este maneja la memoria. Los distintos tipos de formas que tenemos de acceder a ella (memory allocation, memory stack) entro otras cosas.

Se puede comprobar más en detalle en el siguiente enlace: https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html

Lo mismo ocurre con la inmutabilidad y la mutabilidad. Hay que tener en cuenta que a grandes rasgos, rust es muy quisquilloso con el como y el cuando accedemos y podemos modificar las variables.

Tanto como para acceder a ellas, como al momento de pasarlas como parámetro, etc... 

Se tiene que entender que a diferencia de otros lenguajes, rust te obliga a ser muy especifico a la hora de definir cuando se crea una variable, como se accede a ella, como y cuando se puede mutar y donde se alojará esta (stack o allocate). Todos estos aspectos hacen que rust sea un lenguaje bastante rápido y seguro ya que tiene muchos mecanismos para que se maneje de manera correcta los recursos del lenguaje.
