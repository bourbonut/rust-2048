// mod colors;

//mod game;
//mod graphics;

//fn main() {
//    let colors = colors::Colors::new();
//    println!("{:?}", colors.get(&(-1)));
//}

mod colors;

use ggez::conf::WindowMode;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::glam::*;
use ggez::graphics::{
    Canvas, DrawMode, Drawable, FontData, Mesh, PxScale, Rect, Text, TextFragment,
};
use ggez::{Context, GameResult};
use std::env::current_dir;
use std::path::PathBuf;

use colors::{GameColor, BACKGROUND, SPRITES};

struct MainState {
    pos_x: f32,
    locations: Vec<Vec2>,
    background: GameColor,
    game_colors: [GameColor; 18],
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = FontData::from_path(&ctx.fs, PathBuf::from("/clear-sans.bold.ttf"))?;
        ctx.gfx.add_font("ClearSans-Bold", font);

        let s = MainState {
            pos_x: 0.0,
            locations: (0..4)
                .flat_map(|i| {
                    (0..4).map(move |j| Vec2::new((15 + 121 * j) as f32, (15 + 121 * i) as f32))
                })
                .collect(),
            background: GameColor::new(BACKGROUND),
            game_colors: SPRITES.map(|sprite| GameColor::new(sprite)),
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, self.background.rgb);

        for (i, location) in self.locations.iter().enumerate() {
            let number = 2_i64.pow(i as u32 + 1);
            let game_color = &self.game_colors[i + 1];
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
            canvas.draw(&rect, *location);
            canvas.draw(
                &text,
                *location + Vec2::new((53 - w as i32 - 2) as f32, (53 - h as i32 - 5) as f32),
            );
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let current_path = current_dir().unwrap();
    let resources_path = current_path.join(PathBuf::from("resources"));
    if !resources_path.exists() {
        panic!("\nIn order to access correctly to resources, you must run the following command\
            from the root of the project:\n$ cargo run\n")
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
