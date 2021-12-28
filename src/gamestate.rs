use game::arena::Arena;
use game::block::BlockType;
use ggez::Context;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::{event::EventHandler, GameResult, timer, graphics};
use ggez::graphics::{Color, Mesh, DrawMode, MeshBuilder, DrawParam};
use glam::Vec2;
use crate::BACKGROUND_COLOR;
use crate::utils::Atlas;
use crate::viewport::Viewport;


// the ticks per second for the physics simulation.
const DESIRED_FPS: u32 = 60;

#[derive(Debug)]
pub struct GameState {
    arena: Arena,
    mapmesh: Mesh,
    atlas: Atlas,
}


/// builds a mapmesh from a given arena.
fn build_mapmesh(arena: &Arena, ctx: &mut Context) -> GameResult<Mesh> {
    let mb = &mut MeshBuilder::new();
    let colors = [Color::BLACK, Color::BLUE];
    for i in 0..arena.blockrects.len() {
        let blockrects = &arena.blockrects[i];
        for rect in blockrects {
            let r = ggez::graphics::Rect {x: rect.x, y: rect.y, w: rect.w, h: rect.h};
            mb.rectangle(DrawMode::stroke(2.0), r, colors[i])?;
        }
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
            if keyboard::is_key_pressed(ctx, KeyCode::W) {
                self.arena.player.position -= Vec2::new(0.0, 10.0);
            }
            if keyboard::is_key_pressed(ctx, KeyCode::A) {
                self.arena.player.position -= Vec2::new(10.0, 0.0);
            }
            if keyboard::is_key_pressed(ctx, KeyCode::S) {
                self.arena.player.position += Vec2::new(0.0, 10.0);
            }
            if keyboard::is_key_pressed(ctx, KeyCode::D) {
                self.arena.player.position += Vec2::new(10.0, 0.0);
            }

            self.arena.update(dt);
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
