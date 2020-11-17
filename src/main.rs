use toggle_florp::random_color;

fn main() {
    let color = random_color();
    println!(
        "The color of the florp shall be:\n{} {} {}",
        color.0,
        color.1,
        color.2,
    );

}
