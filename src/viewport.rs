use game::{player::Player, arena::{ARENA_WIDTH, ARENA_HEIGHT}};
use ggez::{Context, graphics};
use glam::Vec2;

#[derive(Debug)]
pub struct Viewport {
    pub offset: Vec2
}

impl Viewport {
    pub fn new(offset: Vec2) -> Self {
        Self { offset }
    }

    /// calculates the location of the viewport (where we want the screen to display).
    /// This will be generally centered around the player, unless the player is at a corner or edge
    /// of the map.
    ///
    /// If screen is bigger than the game view, then we center the game map on the center of the
    /// screen.
    pub fn get_viewport_centered_at(point: Vec2, ctx: &Context) -> Viewport {
        let (screen_width, screen_height): (f32, f32) = graphics::size(ctx);
        let mut screen_corner = point - Vec2::new(screen_width, screen_height) / 2.0;

        let topleft = Vec2::ZERO;
        let bottomright_x = f32::max(ARENA_WIDTH - screen_width, 0.0);
        let bottomright_y = f32::max(ARENA_HEIGHT - screen_height, 0.0);
        let bottomright = Vec2::new(bottomright_x, bottomright_y);

        screen_corner = screen_corner.clamp(topleft, bottomright);

        // centers if screen is taller or wider.
        let screen_is_wider = (screen_width > ARENA_WIDTH) as u8;
        let screen_is_taller = (screen_height > ARENA_HEIGHT) as u8;
        let centervector_x = (screen_is_wider as f32) * (screen_width - ARENA_WIDTH) / 2.0;
        let centervector_y = (screen_is_taller as f32) * (screen_height - ARENA_HEIGHT) / 2.0;
        screen_corner -= Vec2::new(centervector_x, centervector_y);

        Viewport::new(screen_corner)
    }

    /// Same as `get_viewport_centered_at(point, &ctx)` function, but centers it on the player
    /// instead.
    pub fn get_viewport_on_player(player: &Player, ctx: &Context) -> Viewport {
        let playercenter = player.position + Vec2::new(player.width, player.height) / 2.0;
        Viewport::get_viewport_centered_at(playercenter, ctx)
    }

    /// Returns the offset of the viewport.
    pub fn get_offset(&self) -> Vec2 {
        -self.offset
    }
}
