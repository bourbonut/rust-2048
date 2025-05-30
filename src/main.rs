mod colors;
mod game;
mod graphics;

use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::graphics::FontData;
use ggez::event;
use ggez::GameResult;
use graphics::MainState;
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
        .window_mode(WindowMode::default().dimensions(500., 500.)) // vsync(false) to get more FPS
        .window_setup(WindowSetup::default().title("2048").icon("/logo.png"));

    let (mut ctx, event_loop) = cb.build()?;
    let font = FontData::from_path(&ctx.fs, PathBuf::from("/clear-sans.bold.ttf"))?;
    ctx.gfx.add_font("ClearSans-Bold", font);
    let state = MainState::new();
    event::run(ctx, event_loop, state)
}
