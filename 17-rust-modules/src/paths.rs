/*
    LOS PATHS

    Al igual que en los filesystems, hay dos tipso de formas para acceder a los crates dentro del proyecto. 

    - Absolute: Los paths absolutes son los paths que empiezan desde el crate root siendo este la raiz del programa. En el caso de los crates externos, se accede a estos usando el nombre del crate como raiz; y en el caso del crate actual, hacemos uso de "literal crate" haciendo referencia al crate en sí mismo donde se encuentra.

    - Relative: Estos empiezan desde el modulo actual y usa self y super como identificadores.

    En ambos casos van seguidos de identificadores separados por (::).

    
*/