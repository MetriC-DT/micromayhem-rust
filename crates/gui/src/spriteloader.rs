use std::path::Path;
use serde::Deserialize;
use ggez::graphics;
use glam::Vec2;
use std::fs::File;
use std::io::BufReader;

/// structs used to get the sprite images from the spritesheet.
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
    meta: Meta,
}

#[derive(Deserialize, Debug)]
struct Meta {
    size: AtlasSize,
}

#[derive(Deserialize, Debug)]
struct AtlasSize {
    w: i32,
    h: i32,
}

impl Atlas {
    pub fn new(texture_atlas_file: &Path) -> Self {
        let errorstr = format!("Couldn't find the texture_atlas file {:?}", texture_atlas_file);
        let file = File::open(texture_atlas_file).expect(&errorstr);
        let buf_reader = BufReader::new(file);
        let formatted = format!("Couldn't create texture atlas from {:?}", texture_atlas_file);
        serde_json::from_reader(buf_reader).expect(&formatted)
    }

    /// Returns a sprite from the Atlas.
    pub fn create_sprite(&self, sprite_name: &str, size: Vec2) -> Sprite {
        let width = self.meta.size.w as f32;
        let height = self.meta.size.h as f32;
        let atlas_rect = graphics::Rect::new(0.0, 0.0, width, height);

        if let Some(sprite_data) = self.frames.iter().find(|d| d.filename == sprite_name) {
            let x = sprite_data.frame.x as f32;
            let y = sprite_data.frame.y as f32;
            let w = sprite_data.frame.w as f32;
            let h = sprite_data.frame.h as f32;

            Sprite::new(
                graphics::Rect::fraction(x, y, w, h, &atlas_rect),
                Vec2::ONE,
            )
        } else {
            panic!("Cannot find sprite {}", sprite_name);
        }
    }
}

#[derive(Clone, Debug)]
/// The square that we want to cut out of the texture atlas.
pub struct Sprite {
    pub rect: graphics::Rect,
    pub scale: Vec2,
}

impl Sprite {
    pub fn new(rect: graphics::Rect, scale: Vec2) -> Self {
        Self { rect, scale }
    }

    pub fn draw_to(&self, pos: Vec2) -> graphics::DrawParam {
        graphics::DrawParam::new()
            .src(self.rect)
            .scale(self.scale)
            .dest(pos)
    }
}
