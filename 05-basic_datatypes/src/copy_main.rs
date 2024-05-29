// Conceptos básicos en rust
use std::time::{Instant};

fn main (){

    let start_time = Instant::now();

    for _i in 1..=1000000{
        println!("")
    }

    let end_time = Instant::now();
    println!("{:?} en rust",end_time - start_time);
}
