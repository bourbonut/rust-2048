use std::path::PathBuf;
use ggez::event;
use ggez::glam::*;
use ggez::graphics::{
    Canvas, DrawMode, Drawable, FontData, Mesh, PxScale, Rect, Text, TextFragment,
};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

use crate::colors::{GameColor, BACKGROUND, SPRITES};
use crate::game::Game;

struct Cell {
    rect: Mesh,
    text: Text,
}

impl Cell {
    fn new(ctx: &mut Context, number: u32, game_color: GameColor) -> GameResult<Self> {
        let number = if number != 0 {
            2_i64.pow(number)
        } else {
            0_i64
        };
        let rect = Mesh::new_rounded_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0., 0., 105., 105.),
            5.,
            game_color.rgb,
        )?;
        let text = Text::new(
            TextFragment::new(format!("{}", number))
                .font("ClearSans-Bold")
                .color(game_color.font_color)
                .scale(PxScale::from(game_color.size as f32)),
        );
        Ok(Cell { rect, text })
    }
}


pub struct MainState {
    game: Game,
    key: i8,
    locations: Vec<Vec2>,
    background: GameColor,
    cells: [Cell; 18],
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = FontData::from_path(&ctx.fs, PathBuf::from("/clear-sans.bold.ttf"))?;
        ctx.gfx.add_font("ClearSans-Bold", font);

        let mut number: u32 = 0;
        let s = MainState {
            game: Game::new(),
            key: 0,
            locations: (0..4)
                .flat_map(|i| {
                    (0..4).map(move |j| Vec2::new((15 + 121 * j) as f32, (15 + 121 * i) as f32))
                })
                .collect(),
            background: GameColor::new(BACKGROUND),
            cells: SPRITES.map(|sprite| {
                let cell = Cell::new(ctx, number, GameColor::new(sprite)).unwrap();
                number += 1;
                cell
            }),
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.key != 0 {
            self.game.action(self.key, true);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, self.background.rgb);

        for (i, location) in self.locations.iter().enumerate() {
            let cell = &self.cells[i];
            let rect = &cell.rect;
            let text = &cell.text;
            let [w, h] = text.dimensions(ctx).unwrap().center().into();
            canvas.draw(rect, *location);
            canvas.draw(
                text,
                *location + Vec2::new((53 - w as i32 - 2) as f32, (53 - h as i32 - 5) as f32),
            );
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> Result<(), ggez::GameError> {
        if let Some(keycode) = input.keycode {
            match keycode {
                KeyCode::Up => {
                    self.key = -4;
                },
                KeyCode::Down => {
                    self.key = 4;
                },
                KeyCode::Left => {
                    self.key = -1;
                },
                KeyCode::Right => {
                    self.key = 1;
                },
                _ => {
                    self.key = 0;
                }
            }
        } else {
            self.key = 0;
        }
        Ok(())
    }
}
