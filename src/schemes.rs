use ggez::graphics::Color;
use std::array::from_fn;
use std::{fs::File, io::Read};
use yaml_rust2::{yaml::Yaml, YamlLoader};

#[derive(Debug)]
pub struct SpriteDesc {
    pub rgb: Color,
    pub size: f32,
    pub font_color: Color,
}

pub struct Scheme {
    doc: Yaml,
}

impl Scheme {
    pub fn new() -> Scheme {
        let mut file = File::open("../colors.yaml").unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        let docs = YamlLoader::load_from_str(&s).unwrap();

        Scheme {
            doc: docs[0].clone(),
        }
    }

    pub fn get(&self, number: &i64) -> Option<SpriteDesc> {
        let i = Yaml::Integer(*number);
        match &self.doc {
            Yaml::Hash(hash) => {
                if let Some(desc) = &hash.get(&i) {
                    // background rgb
                    let rgb = &desc[0];
                    let [br, bg, bb] = from_fn(|i| rgb[i].as_i64().unwrap() as u8);

                    // scale factor
                    let size = &desc[1];
                    let size = 1.3 * size.as_i64().unwrap() as f32;

                    // font color
                    let font_color = &desc[2];
                    let [fr, fg, fb] = from_fn(|i| font_color[i].as_i64().unwrap() as u8);

                    Some(SpriteDesc {
                        rgb: Color::from_rgb(br, bg, bb),
                        size,
                        font_color: Color::from_rgb(fr, fg, fb),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
