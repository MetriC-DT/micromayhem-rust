use game::arena::Arena;
use game::block::BlockRect;
use game::player::{InputMask, Input};
use ggez::Context;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::{event::EventHandler, GameResult, timer, graphics};
use ggez::graphics::{Color, Mesh, DrawMode, MeshBuilder, DrawParam, Rect};
use glam::Vec2;
use crate::BACKGROUND_COLOR;
use crate::viewport::Viewport;
use gui::spriteloader::Atlas;


// the ticks per second for the physics simulation.
const DESIRED_FPS: u32 = 60;

#[derive(Debug)]
pub struct GameState {
    arena: Arena,
    mapmesh: Mesh,
    atlas: Atlas,
}


/// builds a mapmesh from a given arena.
///
/// TODO: Do not want it to crash when an empty map is inputted.
fn build_mapmesh(arena: &Arena, ctx: &mut Context) -> GameResult<Mesh> {
    let mb = &mut MeshBuilder::new();
    let colors = [Color::BLACK, Color::BLUE];

    for blockitem in arena.get_blocks_iter() {
        let block: BlockRect = blockitem.into();
        let r = Rect{x: block.x, y: block.y, w: block.w, h: block.h};
        mb.rectangle(DrawMode::stroke(1.0), r, colors[block.blocktype as usize]).unwrap();
    }
    mb.build(ctx)
}

impl GameState {
    pub fn new(arena: Arena, ctx: &mut Context, atlas: Atlas) -> GameState {
        let mapmesh = build_mapmesh(&arena, ctx).unwrap();
        GameState {arena, mapmesh, atlas}
    }
}


impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Rely on ggez's built-in timer for deciding when to update the game, and how many times.
        // If the update is early, there will be no cycles, otherwises, the logic will run once for each
        // frame fitting in the time since the last update.
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);
            let mut inputmask = InputMask::new();

            // TODO - load custom hotkey file so we are not locked to WASD...
            if keyboard::is_key_pressed(ctx, KeyCode::W) {
                inputmask.add_mask(Input::Up);
                self.arena.player.position -= Vec2::new(0.0, 10.0);
            }

            if keyboard::is_key_pressed(ctx, KeyCode::A) {
                inputmask.add_mask(Input::Left);
                self.arena.player.position -= Vec2::new(10.0, 0.0);
            }

            if keyboard::is_key_pressed(ctx, KeyCode::S) {
                inputmask.add_mask(Input::Down);
                self.arena.player.position += Vec2::new(0.0, 10.0);
            }

            if keyboard::is_key_pressed(ctx, KeyCode::D) {
                inputmask.add_mask(Input::Right);
                self.arena.player.position += Vec2::new(10.0, 0.0);
            }

            self.arena.update(dt, inputmask);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // color background
        graphics::clear(ctx, Color::from_rgb_u32(BACKGROUND_COLOR));

        // gets new viewport to find where to position the camera.
        let player = &self.arena.player;
        let viewport: Viewport = Viewport::get_viewport_on_player(player, ctx);
        let offset = viewport.get_offset();

        // draws everything else.
        graphics::draw(ctx, &self.mapmesh, DrawParam::default().dest(offset))?;

        let [x, y] = player.position.to_array();
        let playerrect = ggez::graphics::Rect {x, y, w: player.width, h: player.height};
        let meshrect = Mesh::new_rectangle(ctx, DrawMode::fill(), playerrect, Color::BLUE).unwrap();
        graphics::draw(ctx, &meshrect, DrawParam::default().dest(offset))?;

        graphics::present(ctx)
    }
}
