use game::arena::Arena;
use ggez::Context;
use ggez::{event::EventHandler, GameResult, timer, graphics};
use ggez::graphics::{Color, Mesh, DrawMode, MeshBuilder, DrawParam};
use crate::BACKGROUND_COLOR;
use crate::utils::Atlas;


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
    for blockrects in &arena.blockrects {
        for rect in blockrects {
            let r = ggez::graphics::Rect {x: rect.x, y: rect.y, w: rect.w, h: rect.h};
            mb.rectangle(DrawMode::stroke(1.0), r, Color::BLACK)?;
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
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        println!("{}", timer::fps(ctx));
        // color background
        graphics::clear(ctx, Color::from_rgb_u32(BACKGROUND_COLOR));
        graphics::draw(ctx, &self.mapmesh, DrawParam::default())?;

        graphics::present(ctx)
    }
}
