use std::path::Path;
use serde::Deserialize;
use ggez::graphics;
use glam::Vec2;

/// structs used to get the sprite images from the spritesheet.
#[derive(Debug, Deserialize)]
pub struct JsonRect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    size: AtlasSize,
}

#[derive(Debug, Deserialize)]
pub struct AtlasSize {
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


impl Atlas {
    pub fn parse_atlas_json(texture_atlas_file: &Path) -> Self {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(texture_atlas_file).expect("Couldn't find the texture_atlas file");
        let buf_reader = BufReader::new(file);
        let formatted = format!("Couldn't create texture atlas from {:?}", texture_atlas_file);
        serde_json::from_reader(buf_reader).expect(&formatted)
    }

    /// Returns a sprite from the Atlas.
    pub fn create_sprite(&self, sprite_name: &str) -> Sprite {
        let width = self.meta.size.w as f32;
        let height = self.meta.size.h as f32;
        let atlas_rect = graphics::Rect::new(0.0, 0.0, width, height);

        if let Some(sprite_data) = self.frames.iter().find(|d| d.filename == sprite_name) {
            Sprite::new(
                graphics::Rect::fraction(
                    sprite_data.frame.x as f32,
                    sprite_data.frame.y as f32,
                    sprite_data.frame.w as f32,
                    sprite_data.frame.h as f32,
                    &atlas_rect,
                ),
                Vec2::ONE,
                sprite_data.frame.w as f32,
                sprite_data.frame.h as f32,
            )
        } else {
            unimplemented!("Not handling failure to find sprite");
        }
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

impl Sprite {
    pub fn new(rect: graphics::Rect, scale: Vec2, width: f32, height: f32) -> Self {
        Self { rect, scale, width, height }
    }

    /// Adds a draw command to the sprite batch.
    pub fn add_draw_param(&mut self, pos: Vec2) -> graphics::DrawParam {
        self.draw_params(pos)
    }

    pub fn draw_params(&self, pos: Vec2) -> graphics::DrawParam {
        graphics::DrawParam::new()
            .src(self.rect)
            .scale(self.scale)
            .dest(pos)
    }

    /// Returns the bounding box for this sprite.
    pub fn get_bound_box(&self) -> graphics::Rect {
        let mut r = graphics::Rect::new(0.0, 0.0, self.width, self.height);
        r.scale(self.scale.x, self.scale.y);
        r
    }
}
