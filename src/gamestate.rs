use ggez::{event::EventHandler, GameError, graphics::{Color, self}, timer};

pub struct GameState { }

impl GameState {
    pub fn new() -> GameState {
        GameState {}
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let dt: f32 = timer::delta(ctx).as_secs_f32();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let dt: f32 = timer::delta(ctx).as_secs_f32();
        graphics::clear(ctx, Color::BLACK);

        graphics::present(ctx)
    }
}
