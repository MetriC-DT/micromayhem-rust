use std::path::Path;
use serde::Deserialize;
use ggez::graphics;
use glam::Vec2;

/// obtains the minimum of two f32
pub fn min_float(a: f32, b: f32) -> f32 {
    return (a + b - (a - b).abs()) / 2.0;
}

/// obtains the maximum of two f32
pub fn max_float(a: f32, b: f32) -> f32 {
    return (a + b + (a - b).abs()) / 2.0;
}

#[derive(Debug, Deserialize)]
pub struct JsonRect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Debug, Deserialize)]
pub struct SpriteData {
    filename: String,
    frame: JsonRect
}

#[derive(Debug, Deserialize)]
pub struct Atlas {
    frames: Vec<SpriteData>,
}


impl Atlas {
    pub fn parse_atlas_json(texture_atlas_file: &Path) -> Self {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(texture_atlas_file).expect("Couldn't find the texture_atlas file");
        let buf_reader = BufReader::new(file);
        serde_json::from_reader(buf_reader).expect("Couldn't create texture atlas")
    }

}

#[derive(Clone, Debug)]
pub struct Sprite {
    /// The square that we want to cut out of the texture atlas.
    pub rect: graphics::Rect,
    pub scale: Vec2,
    pub width: f32,
    pub height: f32,
}
