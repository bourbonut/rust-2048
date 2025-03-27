// mod colors;

//mod game;
//mod graphics;

//fn main() {
//    let colors = colors::Colors::new();
//    println!("{:?}", colors.get(&(-1)));
//}

mod colors;
mod graphics;

use graphics::MainState;
use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::GameResult;
use std::env::current_dir;
use std::path::PathBuf;

pub fn main() -> GameResult {
    let current_path = current_dir().unwrap();
    let resources_path = current_path.join(PathBuf::from("resources"));
    if !resources_path.exists() {
        panic!(
            "\nIn order to access correctly to resources, you must run the following command\
            from the root of the project:\n$ cargo run\n"
        )
    }
    let cb = ggez::ContextBuilder::new("game2048", "bourbonut")
        .add_resource_path(resources_path)
        .window_mode(WindowMode::default().dimensions(500., 500.))
        .window_setup(
            WindowSetup::default().title("2048"),
            //.icon("/logo.png"),
        );

    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
