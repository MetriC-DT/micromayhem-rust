use game::player::Player;
use game::{ARENA_WIDTH, ARENA_HEIGHT};
use game::arena::Arena;
use game::block::BlockRect;
use game::input::{InputMask, Input};
use ggez::Context;
use ggez::event::KeyCode;
use ggez::{event::EventHandler, GameResult, timer, graphics};
use ggez::graphics::{Color, Mesh, DrawMode, MeshBuilder, DrawParam, Rect};
use glam::Vec2;
use crate::BACKGROUND_COLOR;
use crate::viewport::Viewport;
use gui::spriteloader::Atlas;


// the ticks per second for the physics simulation.
const DESIRED_FPS: u32 = 60;
const DT: f32 = 1.0 / DESIRED_FPS as f32;

#[derive(Debug)]
pub struct GameState {
    arena: Arena,
    mapmesh: Mesh,
    atlas: Atlas,
    inputmask: InputMask,
}


/// builds a mapmesh from a given arena.
///
/// TODO: Do not want it to crash when an empty map is inputted.
fn build_mapmesh(arena: &Arena, ctx: &mut Context) -> GameResult<Mesh> {
    let mb = &mut MeshBuilder::new();
    let colors = [Color::BLACK, Color::BLUE];

    for blockitem in arena.get_blocks_iter() {
        let block: BlockRect = blockitem;
        let r = Rect{x: block.x as f32, y: block.y as f32, w: block.w as f32, h: block.h as f32};
        mb.rectangle(DrawMode::stroke(1.0), r, colors[block.blocktype as usize]).unwrap();
    }

    let bounds = Rect{x: 0.0, y: 0.0, w: ARENA_WIDTH as f32, h: ARENA_HEIGHT as f32};
    mb.rectangle(DrawMode::stroke(1.0), bounds, Color::BLACK).unwrap();
    mb.build(ctx)
}

impl GameState {
    pub fn new(arena: Arena, ctx: &mut Context, atlas: Atlas) -> GameState {
        let mapmesh = build_mapmesh(&arena, ctx).unwrap();
        let inputmask = InputMask::new();
        GameState {arena, mapmesh, atlas, inputmask}
    }

    pub fn draw_player(ctx: &mut ggez::Context, player: &Player, offset: Vec2, color: Color) -> GameResult {
        let [x, y] = player.position.to_array();
        let playerrect = ggez::graphics::Rect {x, y, w: player.width, h: player.height};
        let meshrect = Mesh::new_rectangle(ctx, DrawMode::fill(), playerrect, color)?;
        graphics::draw(ctx, &meshrect, DrawParam::default().dest(offset))?;
        Ok(())
    }
}


impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Rely on ggez's built-in timer for deciding when to update the game, and how many times.
        // If the update is early, there will be no cycles, otherwises, the logic will run once for each
        // frame fitting in the time since the last update.
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.arena.update(DT, &self.inputmask);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // color background
        graphics::clear(ctx, Color::from_rgb_u32(BACKGROUND_COLOR));

        // gets new viewport to find where to position the camera.
        let player = self.arena.get_player();
        let viewport: Viewport = Viewport::get_viewport_on_player(player, ctx);
        let offset = viewport.get_offset();

        // draws everything else.
        graphics::draw(ctx, &self.mapmesh, DrawParam::default().dest(offset))?;

        GameState::draw_player(ctx, player, offset, Color::BLUE)?;

        for p in self.arena.get_other_players_iter() {
            GameState::draw_player(ctx, p, offset, Color::GREEN)?;
        }

        // draws bullets
        for (_, bullet) in self.arena.bullets_iterator() {
            let [x, y] = bullet.get_position().to_array();
            let w = 9.0;
            let h = 9.0;
            let b = ggez::graphics::Rect {x, y, w, h};
            let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), b, Color::RED)?;
            graphics::draw(ctx, &mesh, DrawParam::default().dest(offset))?;
        }

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: ggez::event::KeyMods, _repeat: bool) {
        // TODO - load custom hotkey file so we are not locked to WASD...
        match keycode {
            KeyCode::W => self.inputmask.add_mask(Input::Up),
            KeyCode::A => self.inputmask.add_mask(Input::Left),
            KeyCode::S => self.inputmask.add_mask(Input::Down),
            KeyCode::D => self.inputmask.add_mask(Input::Right),
            KeyCode::O => self.inputmask.add_mask(Input::Shoot),
            _ => ()
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: ggez::event::KeyMods) {
        match keycode {
            KeyCode::W => self.inputmask.remove_mask(Input::Up),
            KeyCode::A => self.inputmask.remove_mask(Input::Left),
            KeyCode::S => self.inputmask.remove_mask(Input::Down),
            KeyCode::D => self.inputmask.remove_mask(Input::Right),
            KeyCode::O => self.inputmask.remove_mask(Input::Shoot),
            _ => ()
        }
    }
}
