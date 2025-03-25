// mod colors;

//mod game;
//mod graphics;

//fn main() {
//    let colors = colors::Colors::new();
//    println!("{:?}", colors.get(&(-1)));
//}

use ggez::conf::WindowMode;
use ggez::event;
use ggez::glam::*;
use ggez::graphics::Drawable;
use ggez::graphics::FontData;
use ggez::graphics::PxScale;
use ggez::graphics::TextFragment;
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};
use std::fs::File;
use std::io::Read;

struct MainState {
    pos_x: f32,
    locations: Vec<Vec2>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut file = File::open("../clear-sans.bold.ttf")?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let font = FontData::from_vec(buf)?;
        ctx.gfx.add_font("ClearSans-Bold", font);
        let s = MainState {
            pos_x: 0.0,
            locations: (0..4)
                .flat_map(|i| {
                    (0..4).map(move |j| Vec2::new((15 + 121 * j) as f32, (15 + 121 * i) as f32))
                })
                .collect(),
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
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(183, 173, 160));

        for location in &self.locations {
            let rect = graphics::Mesh::new_rounded_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 105., 105.),
                5.,
                Color::from_rgb(238, 228, 218),
            )?;
            let text = graphics::Text::new(
                TextFragment::new("2")
                    .font("ClearSans-Bold")
                    .scale(PxScale::from(1.2 * 56.0)),
            );
            let [w, h] = text.dimensions(ctx).unwrap().center().into();
            canvas.draw(&rect, *location);
            canvas.draw(&text, *location + Vec2::new(53. - w, 53. - h - 5.));
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(WindowMode::default().dimensions(500., 500.));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
