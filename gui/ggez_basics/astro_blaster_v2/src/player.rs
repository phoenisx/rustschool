use ggez::graphics;
use ggez::event;
use ggez::nalgebra;
use ggez::{Context, GameResult};
use ggez::input;

use crate::constants;

// keeping it a little smaller that 0.5 scaled size of player image
// pub const PLAYER_BBOX: f32 = 24.0;

#[derive(Debug)]
pub struct Player {
    image: graphics::Image,
    pos: nalgebra::Point2<f32>,
    offset: nalgebra::Vector2<f32>,
    bbox: graphics::Rect,
    // Player's Facing Direction
    facing: f32,
}

impl Player {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let image = graphics::Image::new(ctx, "/playerShip1_blue.png")?;
        let pos = nalgebra::Point2::new(72.0, 0.0);
        let y_offset = constants::VIEWPORT_HEIGHT - image.height() as f32;
        // While Drawing, we will use DrawParams `params`, which contains the actual position,
        // so bbox doesn't require to have the x,y set to pos, we can set x,y to 0,0
        let bbox = graphics::Rect::new(0., 0., image.width() as f32, image.height() as f32);
        Ok(Player {
            image,
            pos,
            offset: nalgebra::Vector2::new(0.0, y_offset),
            bbox,
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
            dy = 1.0; // 1 pixel delta for each update.

        }
        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Right) {
            dx = 1.0;
        }
        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Down) {
            dy = -1.0;
        }
        if input::keyboard::is_key_pressed(ctx, event::KeyCode::Left) {
            dx = -1.0;
        }
        self.pos = nalgebra::Point2::new(self.pos.x + dx, self.pos.y + dy);
        self.bbox = graphics::Rect::new(self.pos.x, self.pos.y, self.image.width() as f32, self.image.height() as f32);
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
        let reverted_pos = nalgebra::Point2::new(
            self.offset.x + self.pos.x,
            self.offset.y - self.pos.y
        );
        let params = graphics::DrawParam::new()
            .dest(reverted_pos);
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.0),
            self.bbox,
            graphics::Color::new(0.8, 0.2, 0.3, 1.0)
        )?;

        graphics::draw(ctx, &self.image, params)?;
        graphics::draw(ctx, &mesh, params)?;

        Ok(())
    }
}
