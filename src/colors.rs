use std::ops::Index;

use ggez::graphics::Color;

pub const BACKGROUND: ([u8; 3], u32, [u8; 3]) = ([183, 173, 160], 0, [119, 110, 101]);
// background color, size, font color
pub const GAMEDATA: [([u8; 3], u32, [u8; 3]); 18] = [
    ([205, 193, 180], 0, [119, 110, 101]),  // 0
    ([238, 228, 218], 56, [119, 110, 101]), // 2
    ([237, 224, 200], 56, [119, 110, 101]), // 4
    ([242, 177, 121], 56, [255, 255, 255]), // 8
    ([245, 149, 99], 56, [255, 255, 255]),  // 16
    ([246, 124, 95], 56, [255, 255, 255]),  // 32
    ([246, 94, 59], 56, [255, 255, 255]),   // 64
    ([237, 207, 114], 56, [255, 255, 255]), // 128
    ([237, 204, 97], 56, [255, 255, 255]),  // 256
    ([237, 200, 80], 56, [255, 255, 255]),  // 512
    ([237, 197, 63], 42, [255, 255, 255]),  // 1024
    ([237, 194, 46], 42, [255, 255, 255]),  // 2048
    ([59, 58, 53], 42, [255, 255, 255]),    // 4096
    ([59, 58, 53], 42, [255, 255, 255]),    // 8192
    ([59, 58, 53], 34, [255, 255, 255]),    // 16384
    ([59, 58, 53], 34, [255, 255, 255]),    // 32768
    ([59, 58, 53], 34, [255, 255, 255]),    // 65536
    ([59, 58, 53], 26, [255, 255, 255]),    // 131072
];

pub fn as_color(rgb: [u8; 3]) -> Color {
    let [r, g, b] = rgb;
    Color::from_rgb(r, g, b)
}


#[derive(Debug)]
pub struct GameColor {
    pub rgb: Color,
    pub size: f32,
    pub font_color: Color,
}

impl From<([u8; 3], u32, [u8; 3])> for GameColor {
    fn from(item: ([u8; 3], u32, [u8; 3])) -> Self {
        Self {
            rgb: as_color(item.0),
            size: 1.3 * item.1 as f32,
            font_color: as_color(item.2),
        }
    }
}

impl GameColor {
    pub fn scale(&self, factor: f32, base: f32) -> GameColor {
        let q = self.size.div_euclid(base);
        let r = self.size.rem_euclid(base) / base;
        Self {
            rgb: self.rgb,
            size: factor * (q + r),
            font_color: self.font_color,
        }
    }
}

#[derive(Debug)]
pub struct GameColors([GameColor; 18]);

impl GameColors {
    pub fn new() -> Self {
        Self(GAMEDATA.map(|game_data| GameColor::from(game_data)))
    }
}

impl Index<&u32> for GameColors {
    type Output = GameColor;
    fn index(&self, number: &u32) -> &Self::Output {
        let index = if *number == 0 {
            0
        } else {
            let n = (*number as f32).log2();
            n as usize
        };
        &self.0[index]
    }
}
