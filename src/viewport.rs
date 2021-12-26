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
    pub fn get_viewport(player: &Player, ctx: &Context) -> Viewport {
        let (screen_width, screen_height): (f32, f32) = graphics::size(ctx);
        let playercenter = player.position + Vec2::new(player.width, player.height) / 2.0;

        let screen_corner = playercenter - Vec2::new(screen_width, screen_height) / 2.0;

        let topleft = Vec2::ZERO;
        let bottomright_x = f32::max(ARENA_WIDTH - screen_width, 0.0);
        let bottomright_y = f32::max(ARENA_HEIGHT - screen_height, 0.0);
        let bottomright = Vec2::new(bottomright_x, bottomright_y);

        let screen_corner = screen_corner.clamp(topleft, bottomright);
        Viewport::new(screen_corner)
    }
}
