use std::path::PathBuf;
use std::u32;
use std::usize;
use ggez::event;
use ggez::glam::*;
use ggez::graphics::{
    Canvas, DrawMode, Drawable, FontData, Mesh, PxScale, Rect, Text, TextFragment,
};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

use crate::colors::{GameColor, BACKGROUND, SPRITES};
use crate::game::{Game, ORDERS};

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
    has_moved: bool,
    before_grid: [u32; 16],
    after_grid: [u32; 16],
    moves: Vec<(usize, usize, u32)>,
    additions: Vec<(usize, u32)>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = FontData::from_path(&ctx.fs, PathBuf::from("/clear-sans.bold.ttf"))?;
        ctx.gfx.add_font("ClearSans-Bold", font);

        let mut number: u32 = 0;
        let game = Game::init_first_elements();
        let s = MainState {
            before_grid: game.copy_grid(),
            after_grid: game.copy_grid(),
            game,
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
            has_moved: false,
            moves: Vec::new(),
            additions: Vec::new(),
        };
        Ok(s)
    }

    fn update_moves(&mut self, directions: [[usize; 4]; 4]) {
        for direction in directions {
            let before = direction.map(|i| self.before_grid[i]);
            let after = direction.map(|i| self.after_grid[i]);
            self.moving(before, after, direction);
        }
    }

    fn moving(&mut self, before: [u32; 4], after: [u32; 4], direction: [usize; 4]) {
        let mut var_i: i8 = 3;
        let mut before = before;
        while var_i > -1 {
            let i = var_i as usize;
            if after[i] != 0 {
                break;
            } else if after[i] == before[i] {
                var_i -= 1;
                continue;
            }
            let reference = after[i];
            let mut var_j = var_i.clone();
            while var_j > -1 {
                let j = var_j as usize;
                let item = before[j];
                if item == reference {
                    self.moves.push((direction[j], direction[i], before[j]));
                    break;
                } else if item != 0 && item < reference {
                    let mut k = 1;
                    let mut found = false;
                    while k < j + 1 {
                        if before[j - k] != 0 && before[j - k] != item {
                            break;
                        } else if before[j - k] == item {
                            self.additions.push((direction[i], reference));
                            if i != j {
                                self.moves.push((direction[j], direction[i], item));
                            }
                            self.moves.push((direction[j - k], direction[i], item));
                            let mut idx = 0;
                            before = before.map(|x| {
                                let r = if idx < j - k { x } else { 0 };
                                idx += 1;
                                r
                            });
                            found = true;
                            break;
                        }
                        k += 1;
                    }
                    if found {
                        break;
                    }
                }
                var_j -= 1;
            }
            var_i -= 1;
        }
    }

    //fn prepare_movements(&self) -> Vec<(Cell, (Vec))>

    //fn animation(&mut self, directions: [[usize; 4]; 4]) { }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if !self.game.is_gameover() && self.has_moved {
            self.game.random();
            self.has_moved = false;
        }
        while ctx.time.check_update_time(8) {
            if !self.game.is_gameover() {
                if self.key != 0 {
                    if self.game.partial_move(self.key) {
                        self.before_grid = self.game.copy_grid();
                        self.game.move_zero(&ORDERS[&self.key]);
                        self.game.compare(&ORDERS[&(-self.key)]);
                        self.game.move_zero(&ORDERS[&self.key]);
                        self.after_grid = self.game.copy_grid();
                        self.has_moved = true;
                        self.key = 0;
                    }
                }
            }
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
