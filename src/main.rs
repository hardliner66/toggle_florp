use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let r: u8 = rng.gen();
    let g: u8 = rng.gen();
    let b: u8 = rng.gen();
    println!("The color of the florp shall be:\n{} {} {}", r, g, b);
}
