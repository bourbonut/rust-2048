mod colors;

//mod game;
//mod graphics;

fn main() {
    let colors = colors::Colors::new();
    println!("{:?}", colors.get(&(-1)));
}
