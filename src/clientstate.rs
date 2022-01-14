use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

use game::player::Player;
use game::arena::Arena;
use game::input::{InputMask, Input};
use ggez::Context;
use ggez::event::KeyCode;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::{event::EventHandler, GameResult, timer, graphics};
use ggez::graphics::{Color, Mesh, DrawMode, DrawParam};
use glam::Vec2;
use gui::spriteloader::Atlas;
use network::client::Client;
use network::message::Message;
use crate::{BACKGROUND_COLOR, TICK_RATE};
use crate::viewport::Viewport;
use std::io::Result;

#[derive(Debug)]
pub struct ClientState {
    client: Client,
    mapmesh: SpriteBatch,
    inputmask: InputMask,
}



impl ClientState {
    pub fn new(ctx: &mut Context, atlas: &Atlas, server: &SocketAddr, name: &str) -> Result<ClientState> {
        let mut client = Client::new(0)?;
        client.connect(server, name)?;

        // keep trying to receive, until timeout.
        let mut tries = 10;
        while *client.try_get_id() == None && tries > 0 {
            client.receive();
            tries -= 1;
            thread::sleep(Duration::from_secs_f32(0.5));
        }

        let arena = client.try_get_arena().expect("Unable to get arena");
        let mapmesh = ClientState::build_mapmesh(&arena, ctx, atlas).unwrap();
        let inputmask = InputMask::new();
        return Ok(ClientState {client, mapmesh, inputmask});
    }

    /// TODO: Use player sprite rather than just a rectangle.
    pub fn draw_player(ctx: &mut ggez::Context, player: &Player, offset: Vec2, color: Color) -> GameResult {
        let [x, y] = player.position.to_array();
        let playerrect = ggez::graphics::Rect {x, y, w: player.width, h: player.height};
        let meshrect = Mesh::new_rectangle(ctx, DrawMode::fill(), playerrect, color)?;
        graphics::draw(ctx, &meshrect, DrawParam::default().dest(offset))?;
        Ok(())
    }

    /// builds a mapmesh from a given arena.
    fn build_mapmesh(arena: &Arena, ctx: &mut Context, atlas: &Atlas) -> GameResult<SpriteBatch> {
        let spritesheet_image = graphics::Image::new(ctx, "/sprites/platforms.png")?;
        let mut spritebatch = graphics::spritebatch::SpriteBatch::new(spritesheet_image);

        // Nearest or Linear
        spritebatch.set_filter(graphics::FilterMode::Nearest);

        for block in arena.get_blocks_iter() {
            let spritename = block.blocktype.to_string() + ".png";
            let (x, y, w, h) = (block.x, block.y, block.w, block.h);
            let dest = Vec2::new(x, y);
            let size = Vec2::new(w, h);

            // cut out image from the spritesheet and resize.
            let sprite_rect = atlas.create_sprite(&spritename, size).draw_to(dest);
            spritebatch.add(sprite_rect);
        }

        Ok(spritebatch)
    }
}


impl EventHandler for ClientState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // Rely on ggez's built-in timer for deciding when to update the game, and how many times.
        // If the update is early, there will be no cycles, otherwises, the logic will run once for each
        // frame fitting in the time since the last update.
        while timer::check_update_time(ctx, TICK_RATE) {
            // sends input
            let input_message = Message::write_input(self.inputmask);
            self.client.send_message(&input_message).unwrap();

            // obtains updated arena.
            self.client.receive();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        // color background
        graphics::clear(ctx, Color::from_rgb_u32(BACKGROUND_COLOR));

        let arena = self.client.try_get_arena().expect("Cannot draw arena");
        let id = self.client.try_get_id().expect("No player ID assigned");

        // gets new viewport to find where to position the camera.
        // TODO: obtain the correct player (not just the first one).
        let player = arena.get_player(id);
        let viewport: Viewport = Viewport::get_viewport_on_player(player, ctx);
        let offset = viewport.get_offset();

        // draws everything else.
        graphics::draw(ctx, &self.mapmesh, DrawParam::default().dest(offset))?;

        ClientState::draw_player(ctx, player, offset, Color::WHITE)?;

        for (_, p) in arena.get_players().iter() {
            ClientState::draw_player(ctx, p, offset, Color::GREEN)?;
        }

        // draws bullets
        for (_, bullet) in arena.bullets_iterator() {
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
            KeyCode::P => self.inputmask.add_mask(Input::Bomb),
            KeyCode::I => self.inputmask.add_mask(Input::Throw),
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
            KeyCode::P => self.inputmask.remove_mask(Input::Bomb),
            KeyCode::I => self.inputmask.remove_mask(Input::Throw),
            _ => ()
        }
    }
}
