use ggez::{conf, event, graphics, Context, ContextBuilder, GameResult};

mod player;
mod constants;
use player::Player;
use constants::{VIEWPORT_WIDTH, VIEWPORT_HEIGHT};

struct World {
    stage: usize, // Nothing else for now.
    player: Player,
}

impl World {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(World {
            stage: 0,
            player: Player::new(ctx)?,
        })
    }
}

impl event::EventHandler for World {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.player.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK); // Clear with Black Background
        self.player.draw(ctx)?;
        graphics::present(ctx)?; // It's important to present the buffer on Screen
        Ok(())
    }
}

fn main() -> GameResult {
    let window_config = conf::Conf::new();
    let win_mode = conf::WindowMode::default();
    let window_config = window_config.window_mode(win_mode.dimensions(
        VIEWPORT_WIDTH, VIEWPORT_HEIGHT
    ));
    // Resource DIR should be relative to root DIR.
    let resource_dir = "./astro_blaster_v2/resources";
    let (ctx, event_loop) = &mut ContextBuilder::new("Astro Blaster v2", "Subroto Biswas")
        .conf(window_config)
        .add_resource_path(resource_dir)
        .build()?;
    let mut state = World::new(ctx)?;
    event::run(ctx, event_loop, &mut state)
}
