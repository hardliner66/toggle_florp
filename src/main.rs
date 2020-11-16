fn main() {
    println!(
        "The color of the florp shall be:\n{} {} {}",
        rand::random::<u8>(), // r
        rand::random::<u8>(), // g
        rand::random::<u8>()  // b
    );
}
