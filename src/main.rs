mod gamestate;
mod configuration;

use ggez::{GameResult, ContextBuilder, event};
use gamestate::GameState;
use game::{self, arena::Arena};
use micromayhem::*;

fn main() -> GameResult {
    let mut g = GameState::new(Arena::new());

    let mut cb = ContextBuilder::new(GAME_TITLE, AUTHOR);
    cb = configuration::load_configuration(cb);

    let (mut ctx, event_loop) = cb.build()?;

    event::run(ctx, event_loop, g);
}
