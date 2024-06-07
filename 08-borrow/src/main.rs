fn main() {
    let mut s = String::from("Hola");

    let m1 = &s;
    let m2 = &s;

    println!("{} {}",m1,m2);
    println!("{} {}",m1,m2);

    let m3 = &mut s;

    m3.push_str(" mundo");
    println!("{}",m3);

    let s2 = change_value(&mut s);

    println!("{}",s2);
}

fn change_value(si:&mut String) -> &String{
    si.push_str(" another one");
    si
}


