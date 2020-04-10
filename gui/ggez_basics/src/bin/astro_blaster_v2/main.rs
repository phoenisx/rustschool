use ggez::event;
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};

struct World {
    stage: usize, // Nothing else for now.
}

impl World {
    fn new() -> Self {
        World {
            stage: 0
        }
    }
}

impl event::EventHandler for World {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK); // Clear with Black Background
        graphics::present(ctx)?; // It's important to present the buffer on Screen
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("Astro Blaster v2", "Subroto Biswas")
        .build()?;
    let mut state = World::new();
    event::run(ctx, event_loop, &mut state)
}
