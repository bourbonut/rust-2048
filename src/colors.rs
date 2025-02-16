use yaml_rust2::{yaml::Yaml, YamlLoader};
use std::{fs::File, io::Read};
use std::array::from_fn;

#[derive(Debug)]
pub struct Color {
    pub rgb: [u8; 3],
    pub size: u32,
    pub font_color: [u8; 3]
}

pub struct Colors {
    doc: Yaml,
}

impl Colors {
    pub fn new() -> Colors {
        let mut file = File::open("../colors.yaml").unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        let docs = YamlLoader::load_from_str(&s).unwrap();

        Colors {
            doc: docs[0].clone(),
        }
    }

    pub fn get(&self, color: &i64) -> Option<Color> {
        let i = Yaml::Integer(*color);
        match &self.doc {
            Yaml::Hash(hash) => {
                if let Some(color_sprite) = &hash.get(&i) {
                    let rgb = &color_sprite[0];
                    let size = &color_sprite[1];
                    let font_color = &color_sprite[2];
                    Some(
                        Color {
                            rgb: from_fn(|i| rgb[i].as_i64().unwrap() as u8),
                            size: size.as_i64().unwrap() as u32,
                            font_color: from_fn(|i| font_color[i].as_i64().unwrap() as u8),
                        }
                    )
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}
