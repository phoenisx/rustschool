use ggez::event::{EventHandler};
use ggez::graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh};
use ggez::{Context, GameResult};
use ggez::nalgebra::{Point2};

use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

pub struct Ball {
    ball: Mesh,
    pos: Point2<f32>
}

impl Ball {
    // Point2::new(WINDOW_WIDTH / 2.0 - (2.0 * dpi_factor), WINDOW_HEIGHT / 2.0 - (2.0 * dpi_factor))
    pub fn new(ctx: &mut Context, radius: f32, init_pos: Point2<f32>) -> GameResult<Self> {
        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;
        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::Fill(FillOptions::default()),
            Point2::new(0.0, 0.0),
            radius * dpi_factor,
            1.0,
            Color::from_rgb(255, 255, 255),
        )?;

        Ok(Ball {
            ball,
            pos: init_pos
        })
    }
}

impl EventHandler for Ball {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.ball, DrawParam::default().dest(self.pos))?;
        Ok(())
    }
}
