///
/// Since Rust is a little different and having global variable is difficult
/// we need a place to have separate instances of players, with paddle images
/// and that counts their own score, and movement.
///
use ggez::event::{EventHandler};
use ggez::graphics::{self, Color, DrawParam, Mesh};
use ggez::{Context, GameResult};
use ggez::nalgebra::{Point2};

pub struct Player {
    paddle: Mesh,
    pos: Point2<f32>,
}

// GGez has some kind of internal code, that even when things are drawn on a smaller
// Canvas, the coordinate system it works with, is for the full-fledged Window size.
impl Player {
    pub fn new(ctx: &mut Context, width: f32, height: f32, init_pos: Point2<f32>) -> GameResult<Self> {
        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;
        let paddle = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::Fill(graphics::FillOptions::default()),
            graphics::Rect::new(0.0, 0.0, width * dpi_factor, height * dpi_factor),
            Color::from_rgb(255, 255, 255),
        )?;

        Ok(Player {
            paddle,
            pos: init_pos,
        })
    }
}

impl EventHandler for Player {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.paddle, DrawParam::default().dest(self.pos))?;
        Ok(())
    }
}
