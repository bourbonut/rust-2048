use ggez::event;
use ggez::glam::*;
use ggez::graphics::{
    Canvas, DrawMode, Drawable, Mesh, PxScale, Rect, Text, TextFragment,
};
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};
use std::usize;

use crate::colors::{GameColors, GameColor, BACKGROUND, as_color};
use crate::game::Game;

// number of how many images will be drawn for an animation
const NB_I: f32 = 8.;

#[derive(Debug)]
pub struct Movement {
    number: usize,
    start: Vec2,
    q: Vec2,
    r: Vec2,
}

fn draw_cell(
    canvas: &mut Canvas,
    ctx: &mut Context,
    number: u32,
    game_color: &GameColor,
    location: Vec2
) -> GameResult<()> {
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
    let [w, h] = text.dimensions(ctx).unwrap().center().into();
    canvas.draw(&rect, location);
    canvas.draw(
        &text,
        location + Vec2::new((53 - w as i32 - 2) as f32, (53 - h as i32 - 5) as f32),
    );
    Ok(())
}

pub struct MainState {
    game: Game,
    key: i8,
    counter_1: u32,
    counter_2: u32,
    locations: Vec<Vec2>,
    background: GameColor,
    game_colors: GameColors,
    has_moved: bool,
    before_grid: [u32; 16],
    after_grid: [u32; 16],
    static_locs: Vec<(usize, u32)>,
    moves: Vec<(usize, usize, u32)>,
    additions: Vec<(usize, u32)>,
    direction: Option<[[usize; 4]; 4]>,
    movements: Vec<Movement>,
}

impl MainState {
    pub fn new() -> Self {
        let game = Game::init_first_elements();

        Self {
            before_grid: game.copy_grid(),
            after_grid: game.copy_grid(),
            game,
            counter_1: NB_I as u32 + 1, // for movement animations
            counter_2: NB_I as u32 + 1, // for addition animations
            key: 0,
            locations: (0..4)
                .flat_map(|i| {
                    (0..4).map(move |j| Vec2::new((15 + 121 * j) as f32, (15 + 121 * i) as f32))
                })
                .collect(),
            background: GameColor::from(BACKGROUND),
            game_colors: GameColors::new(),
            has_moved: false,
            moves: Vec::new(),
            additions: Vec::new(),
            direction: None,
            movements: Vec::new(),
            static_locs: Vec::new(),
        }
    }

    /// Resets animation
    fn reset_animations(&mut self) {
        self.moves.clear();
        self.additions.clear();
        self.movements.clear();
        self.static_locs.clear();
    }

    /// Updates static locations by comparing the grid before and after an action
    fn update_static_locations(&mut self) {
        self.static_locs = self
            .before_grid
            .iter()
            .zip(self.after_grid.iter())
            .enumerate()
            .filter_map(|(loc_i, (&before, &after))| {
                if before == after && before != 0 {
                    Some((loc_i, before))
                } else {
                    None
                }
            })
            .collect();
    }

    /// Updates `self.moves` for movements to animate and `self.additions` for additions to
    /// animate
    fn update_moves(&mut self, directions: [[usize; 4]; 4]) {
        for direction in directions {
            let before = direction.map(|i| self.before_grid[i]);
            let after = direction.map(|i| self.after_grid[i]);
            self.moving(before, after, direction);
        }
    }

    /// Fills `self.moves` and `self.additions` based on one layer of the grid before and after
    /// an action
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

    /// Returns a vector of prepared information for movement animations
    fn prepare_movements(&self) -> Vec<Movement> {
        self.moves
            .iter()
            .map(|&(start, end, number)| {
                let start = self.locations[start];
                let end = self.locations[end];
                let diff = end - start;
                let q = Vec2::new(diff[0] / NB_I, diff[1] / NB_I);
                let r = Vec2::new(diff[0] % NB_I, diff[1] / NB_I) / NB_I;
                Movement {
                    number: number as usize,
                    start,
                    q,
                    r,
                }
            })
            .collect()
    }

    /// Prepare movement animations and addition animations
    fn prepare_animations(&mut self, directions: [[usize; 4]; 4]) {
        self.update_static_locations();
        self.update_moves(directions);
        self.movements = self.prepare_movements();
        self.counter_1 = 0;
        self.counter_2 = 0;
    }

    /// Animates one frame of movement animations
    fn animate_movements(&self, ctx: &mut Context, i: u32) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, self.background.rgb);

        for &location in self.locations.iter() {
            let game_color = &self.game_colors[&0];
            draw_cell(&mut canvas, ctx, 0, game_color, location)?;
        }

        for &(pos, number) in self.static_locs.iter() {
            let location = self.locations[pos];
            let game_color = &self.game_colors[&number];
            draw_cell(&mut canvas, ctx, number, game_color, location)?;
        }

        for movement in self.movements.iter() {
            let location = movement.start + (i as f32) * (movement.q + movement.r);
            let number = movement.number as u32;
            let game_color = &self.game_colors[&number];
            draw_cell(&mut canvas, ctx, number, game_color, location)?;
        }

        for &(pos, number) in self.additions.iter() {
            let location = self.locations[pos];
            let game_color = &self.game_colors[&number].scale(0., NB_I);
            draw_cell(&mut canvas, ctx, number, game_color, location)?;
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    /// Animates one frame of addition animations
    fn animate_additions(&self, ctx: &mut Context, i: u32) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, self.background.rgb);
        for (pos, &number) in self.after_grid.iter().enumerate() {
            let location = self.locations[pos];
            let game_color = &self.game_colors[&number];
            draw_cell(&mut canvas, ctx, number, game_color, location)?;
        }

        for &(pos, number) in self.additions.iter() {
            let location = self.locations[pos];
            let game_color = &self.game_colors[&number].scale(i as f32, NB_I);
            draw_cell(&mut canvas, ctx, number, game_color, location)?;
        }
        canvas.finish(ctx)?;
        Ok(())
    }

    /// Draws the game over
    fn draw_gameover(&self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, self.background.rgb);
        let grid = self.game.copy_grid();
        for (&location, number) in self.locations.iter().zip(grid) {
            let game_color = &self.game_colors[&number];
            draw_cell(&mut canvas, ctx, number, game_color, location)?;
        }

        let text = Text::new(
            TextFragment::new("Game Over")
                .font("ClearSans-Bold")
                .color(as_color([255, 0, 0]))
                .scale(PxScale::from(78.)),
        );

        let [w, h] = text.dimensions(ctx).unwrap().center().into();
        canvas.draw(&text, Vec2::new(250. - w, 250. - h));
        canvas.finish(ctx)?;

        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.game.is_gameover() && self.has_moved {
            self.game.random();
            self.has_moved = false;
        }
        if !self.game.is_gameover() {
            if self.key != 0 {
                if self.game.partial_move(self.key) {
                    self.before_grid = self.game.copy_grid();
                    self.game.action(self.key);
                    self.after_grid = self.game.copy_grid();
                    self.has_moved = true;
                    self.direction = Some(self.game.direction(self.key));
                }
                self.key = 0;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(directions) = self.direction {
            self.prepare_animations(directions);
            self.direction = None;
        } else if self.counter_1 <= NB_I as u32 {
            self.animate_movements(ctx, self.counter_1)?;
            self.counter_1 += 1;
        } else if self.counter_2 <= NB_I as u32 {
            self.animate_additions(ctx, self.counter_2)?;
            self.counter_2 += 1;
        } else if self.game.is_gameover() {
            self.reset_animations();
            self.draw_gameover(ctx)?;
        } else {
            self.reset_animations();
            let mut canvas = Canvas::from_frame(ctx, self.background.rgb);
            let grid = self.game.copy_grid();
            for (&location, number) in self.locations.iter().zip(grid) {
                let game_color = &self.game_colors[&number];
                draw_cell(&mut canvas, ctx, number, game_color, location)?;
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
            self.reset_animations();
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
