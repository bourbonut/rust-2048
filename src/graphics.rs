use crate::game::Game;

use ggez::{Context, GameError, GameResult};
use ggez::event::EventHandler;

//fn sprite(number: u32) {
//    color, size, font_color 
//}

struct Graphics {
    game: Game,
}

impl Graphics {
    fn new() -> GameResult<Graphics> {
        let s = Graphics {
            game: Game::new(),
        };
        Ok(s)
    }
}

impl EventHandler<GameError> for Graphics {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        todo!()
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        todo!()
    }
}
