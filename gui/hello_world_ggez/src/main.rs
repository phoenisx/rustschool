// Learn from https://github.com/ggez/ggez/blob/master/docs/guides/HelloGgez.md

use ggez::*;

struct GameState {
    dt: std::time::Duration,
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        println!("Hello ggez! dt = {}ns", self.dt.subsec_nanos());
        Ok(())
    }
}

pub fn main() {
    let state = &mut GameState { dt: std::time::Duration::new(0, 0) };

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "subroto")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state).unwrap();
}
