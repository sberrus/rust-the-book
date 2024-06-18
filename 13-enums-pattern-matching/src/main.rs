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

// podemos definir métodos al igual que a los structs con la keyword impl

impl IpAddr {
    fn showAddress (&self){
        println!("My ip address is: {}",self.address);
    }
}

// creamos un enum que tiene en cada campo un tipo de dato definido para usar a la hora de
// instanciarlo. De esta forma podemos crear estructuras muy legibles y que sea mucho más explicito
// el dato que se desea almacenar y su namespace
enum IpAddrKind2 {
    V4(String),
    V6(String),
}

// otra de las ventajas uqe tenemos al a hora de usar enums es la de que podemos crear datos
// complejos usando una sintaxis similar al de las tuplas.

enum IpAddrKind3 {
    V4(u8,u8,u8,u8),
    V6(String),
}

// podemos definir un struct como campo de un enum para hacerlo aún más flexible
struct IpV4Addr {}
struct IpV6Addr {}

enum IpAddr2 {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

fn main() {

    //Instanciamos los valores del enum
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // instanciamos IpAddrKind2
    let _home = IpAddrKind2::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrKind2::V6(String::from("::1"));
    
    // instanciamos IpAddrKind3
    let _home2 = IpAddrKind3::V4(127,0,0,1);
    let _loopback2 = IpAddrKind3::V6(String::from("::1"));

    // consumiendo una función
    route(IpAddrKind::V4);

    // construyendo struct con enum interno como campo
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.0.0.1"),
    };

    println!("{:?}", home);

    // llamando a método showAddress()
    home.showAddress();
}

// Podemos definir un enum como tipo de dato de la firma de una función. Por lo que nos ayuda a
// dejar claro que valores explicitos debemos pasarle a la misma para evitar comportamientos
// inesperados.
fn route(_ip_kind:IpAddrKind){}


