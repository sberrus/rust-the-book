/*
 * ENUMS
 *
 * Los enums son una forma que tenemos de definir un dato que solo puede contener los
 * valores que nosotros hayamos definido en el mismo 
 * */

// definiendo un enum para definir versiones de direcciones IP. 
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// definiendo un enum dentro de un struct
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {

    //Instanciamos los valores del enum
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // consumiendo una función
    route(IpAddrKind::V4);

    // construyendo struct con enum interno como campo
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.0.0.1"),
    };

    println!("{:?}", home);
}

// Podemos definir un enum como tipo de dato de la firma de una función. Por lo que nos ayuda a
// dejar claro que valores explicitos debemos pasarle a la misma para evitar comportamientos
// inesperados.
fn route(_ip_kind:IpAddrKind){}
