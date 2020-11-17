pub fn random_color() -> (u8, u8, u8) {
    (
        rand::random::<u8>(), // r
        rand::random::<u8>(), // g
        rand::random::<u8>(), // b
    )
}
