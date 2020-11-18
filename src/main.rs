use toggle_florp::Color;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let msg = if args.is_empty() {
        "random".to_owned()
    } else {
        args.join(" ")
    };

    let Color(r, g, b) = Color::from_message(&msg);

    println!("The color of the florp shall be:\n{} {} {}", r, g, b);
}
