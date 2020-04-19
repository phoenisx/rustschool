///
/// Since Rust is a little different and having global variable is difficult
/// we need a place to have separate instances of players, with paddle images
/// and that counts their own score, and movement.
///
use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{self, Color, DrawParam, Mesh};
use ggez::{timer, Context, GameResult};
use ggez::input::{keyboard};
use ggez::nalgebra::{Point2};

use crate::{WINDOW_HEIGHT};

pub const PADDLE_SPEED: f32 = 400.0;

pub struct Player {
    paddle: Mesh,
    bbox: graphics::Rect,
    pos: Point2<f32>,
    score: u32,
    // For now I am keeping the Keyboard Keys stored for (UP, DOWN)
    // for each instance of player, as players can be multiple, and will
    // use different keys to move up/down
    keys: (KeyCode, KeyCode),
}

// GGez has some kind of internal code, that even when things are drawn on a smaller
// Canvas, the coordinate system it works with, is for the full-fledged Window size.
impl Player {
    pub fn new(ctx: &mut Context, width: f32, height: f32, init_pos: Point2<f32>, keys: Option<(KeyCode, KeyCode)>) -> GameResult<Self> {
        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;
        let bbox = graphics::Rect::new(0.0, 0.0, width * dpi_factor, height * dpi_factor);
        let paddle = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::Fill(graphics::FillOptions::default()),
            bbox.clone(),
            Color::from_rgb(255, 255, 255),
        )?;

        Ok(Player {
            paddle,
            bbox,
            pos: init_pos,
            score: 0,
            keys: if let Some(_keys) = keys { _keys } else { (KeyCode::Up, KeyCode::Down) }
        })
    }

    pub fn get_score_string(&self) -> String {
        self.score.to_string()
    }

    pub fn update(&mut self, ctx: &mut Context, elapsed_time: f32) -> GameResult {
        if keyboard::is_key_pressed(ctx, self.keys.0) {
            // TODO(shub) update with using time delta instead of constant
            let new_y = self.pos.y + -PADDLE_SPEED * elapsed_time;
            self.pos = Point2::new(self.pos.x, new_y.max(0.0));
        } else if keyboard::is_key_pressed(ctx, self.keys.1) {
            let new_y = self.pos.y + PADDLE_SPEED * elapsed_time;
            self.pos = Point2::new(self.pos.x, new_y.min(WINDOW_HEIGHT - self.bbox.h));
        }
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.paddle, DrawParam::default().dest(self.pos))?;
        Ok(())
    }
}
