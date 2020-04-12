use ggez::graphics;
use ggez::event;
use ggez::nalgebra;
use ggez::{Context, GameResult};
use ggez::input;

// keeping it a little smaller that 0.5 scaled size of player image
pub const PLAYER_BBOX: f32 = 24.0;

#[derive(Debug)]
pub struct Player {
    image: graphics::Image,
    pos: nalgebra::Point2<f32>,
    // Player's Facing Direction
    facing: f32,
}

impl Player {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Player {
            image: graphics::Image::new(ctx, "/playerShip1_blue.png")?,
            pos: nalgebra::Point2::new(24.0, 24.0),
            facing: 0.,
        })
    }

    // Since ggez support edge-triggered keyboard events, we will have a workaround to
    // use those events as level-triggered.
    pub fn handle_events(&mut self, ctx: &mut Context) -> GameResult {
        let mut dy = 0.;
        let mut dx = 0.;
        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Up) {
            // For simplicity we will be modifying the position directly, instead of calculating
            // it from velocity or something.
            dy = -1.0; // 1 pixel delta for each update.

        }
        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Right) {
            dx = 1.0;
        }
        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Down) {
            dy = 1.0;
        }
        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Left) {
            dx = -1.0;
        }
        self.pos = nalgebra::Point2::new(self.pos.x + dx, self.pos.y + dy);
        Ok(())
    }
}

// This impl is not necessary, I could've created `draw` and `update`
// methods directly in the above impl, but using traits gives us more
// clarity for what to expect and what methods are essential.
impl event::EventHandler for Player {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.handle_events(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let params = graphics::DrawParam::default()
            .dest(self.pos)
            .scale(nalgebra::Vector2::new(0.5, 0.5));
        graphics::draw(ctx, &self.image, params)?;
        Ok(())
    }
}
