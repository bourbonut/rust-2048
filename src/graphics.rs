use std::collections::HashMap;
use std::path::PathBuf;
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

// number of how many images will be drawn for an animation
const NB_I: f32 = 40.;
// number of how many homotheties will be applied for a number after an
// addition
const NB_H: u32 = 40;

const FPS: u32 = 240;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Movement {
    number: usize,
    start: Vec2,
    limits: Vec2,
    q: Vec2,
    r: Vec2,
    symbol: i8,
}

#[derive(Debug)]
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

    fn draw(&self, canvas: &mut Canvas, ctx: &mut Context, location: Vec2) {
        let [w, h] = self.text.dimensions(ctx).unwrap().center().into();
        canvas.draw(&self.rect, location);
        canvas.draw(
            &self.text,
            location + Vec2::new((53 - w as i32 - 2) as f32, (53 - h as i32 - 5) as f32),
        );
    }
}


pub struct MainState {
    game: Game,
    key: i8,
    counter_1: u32,
    counter_2: u32,
    locations: Vec<Vec2>,
    background: GameColor,
    cells: [Cell; 18],
    has_moved: bool,
    before_grid: [u32; 16],
    after_grid: [u32; 16],
    moves: Vec<(usize, usize, u32)>,
    additions: Vec<(usize, u32)>,
    direction: Option<[[usize; 4]; 4]>,
    movements: Vec<Movement>,
    scales: HashMap<u32, Vec<Cell>>,
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
            counter_1: NB_I as u32 + 1,
            counter_2: NB_I as u32 + 1,
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
            direction: None,
            movements: Vec::new(),
            scales: HashMap::new(),
        };
        Ok(s)
    }

    fn reset_animations(&mut self) {
        self.moves.clear();
        self.additions.clear();
        self.movements.clear();
        self.scales.clear();
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
            if after[i] == 0 {
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

    fn prepare_movements(&self) -> Vec<Movement> {
       self.moves.iter().map(
            |&(start, end, number)| {
                let symbol: i8 = if end > start { 1 } else { -1 };
                let start = self.locations[start];
                let end = self.locations[end];
                let diff = end - start;
                let q = Vec2::new(diff[0] / NB_I, diff[1] / NB_I);
                let r = Vec2::new(diff[0] % NB_I, diff[1] / NB_I) / NB_I;
                let limits = Vec2::new(107., 107.) + 2. * symbol as f32 * q;
                Movement { number: number as usize, start, limits, q, r, symbol }
            }
        ).collect()
    }

    fn prepare_scales(&self, ctx: &mut Context) -> HashMap<u32, Vec<Cell>> {
        let mut scales = HashMap::new();
        for &(_, number) in self.additions.iter() {
            if scales.contains_key(&number) {
                continue;
            }
            let i = if number == 0 { 0 } else {
                let n = (number as f32).log2();
                n as usize
            };
            let mut sprite = SPRITES[i].clone();
            let size = sprite.1;

            let q = size / NB_H;
            let r = (size % NB_H) / NB_H;

            let mut images = Vec::new();
            for s in 0..=NB_H {
                sprite.1 = s * q + s * r;
                let game_color = GameColor::new(sprite);
                images.push(Cell::new(ctx, i as u32, game_color).unwrap());
            }

            scales.insert(number, images);
        }
        scales
    }

    fn prepare_animations(&mut self, ctx: &mut Context, directions: [[usize; 4]; 4]) {
        self.update_moves(directions);
        self.movements = self.prepare_movements();
        self.scales = self.prepare_scales(ctx);
        self.counter_1 = 0;
        self.counter_2 = 0;
    }

    fn animate_movements(&self, ctx: &mut Context, i: u32) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, self.background.rgb);

        for &location in self.locations.iter() {
            self.cells[0].draw(&mut canvas, ctx, location);
        }

        for movement in self.movements.iter() {
            let location = movement.start + (i as f32) * (movement.q + movement.r);
            let number = movement.number;
            let i = if number == 0 { 0 } else {
                let n = (number as f32).log2();
                n as usize
            };
            self.cells[i].draw(&mut canvas, ctx, location);
        }

        for &(pos, number) in self.additions.iter() {
            let location = self.locations[pos];
            self.scales[&number][0].draw(&mut canvas, ctx, location);
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn animate_additions(&self, ctx: &mut Context, i: u32) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, self.background.rgb);
        for &location in self.locations.iter() {
            self.cells[0].draw(&mut canvas, ctx, location);
        }

        for &(pos, number) in self.additions.iter() {
            let location = self.locations[pos];
            self.scales[&number][i as usize].draw(&mut canvas, ctx, location);
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if !self.game.is_gameover() && self.has_moved {
            self.game.random();
            self.has_moved = false;
        }
        while ctx.time.check_update_time(FPS) {
            if !self.game.is_gameover() {
                if self.key != 0 {
                    if self.game.partial_move(self.key) {
                        self.before_grid = self.game.copy_grid();
                        self.game.move_zero(&ORDERS[&self.key]);
                        self.game.compare(&ORDERS[&(-self.key)]);
                        self.game.move_zero(&ORDERS[&self.key]);
                        self.after_grid = self.game.copy_grid();
                        self.has_moved = true;
                        self.direction = Some(ORDERS[&self.key]);
                        self.key = 0;
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(directions) = self.direction {
            self.prepare_animations(ctx, directions);
            self.direction = None;
        } else if self.counter_1 <= NB_I as u32 {
            self.animate_movements(ctx, self.counter_1)?;
            self.counter_1 += 1;
        } else if self.counter_2 <= NB_I as u32 {
            self.animate_additions(ctx, self.counter_2)?;
            self.counter_2 += 1;
        } else {
            self.reset_animations();
            let mut canvas = Canvas::from_frame(ctx, self.background.rgb);
            let grid = self.game.copy_grid();
            for (location, number) in self.locations.iter().zip(grid) {
                let i = if number == 0 { 0 } else {
                    let n = (number as f32).log2();
                    n as usize
                };
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
        }
        Ok(())
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            input: ggez::input::keyboard::KeyInput,
            _repeated: bool,
        ) -> Result<(), ggez::GameError> {
        if self.counter_1 <= NB_I as u32 || self.counter_2 <= NB_I as u32 {
            return Ok(());
        }
        if let Some(keycode) = input.keycode {
            self.key = match keycode {
                KeyCode::Up => -4,
                KeyCode::Down => 4,
                KeyCode::Left => -1,
                KeyCode::Right => 1,
                _ => 0,
            }
        } else {
            self.key = 0;
        }
        Ok(())
    }
}
