#![allow(dead_code, unused_imports)]

/// Proper version of cs50 push.lua
///
/// This lib will mimic the way CS50 `push` lib works.
/// The purpose of this libe is to:
/// 1. Allow to have a Canvas with Virtual Width/Height, which will act as render target
///    that can then be scaled to the Actual Window size to look like a retro game.
///

use ggez::graphics::{self, Canvas, Color, FilterMode};
use ggez::nalgebra::Vector2;
use ggez::{conf, Context, GameResult};

pub struct Push {
    canvas: Canvas,
    scale: Vector2<f32>,
}

impl Push {
    pub fn new(
        ctx: &mut Context,
        virtual_width: f32,
        virtual_height: f32,
        screen_width: f32,
        screen_height: f32,
    ) -> GameResult<Self> {
        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;
        let canvas = Canvas::new(
            ctx,
            virtual_width as u16 * dpi_factor as u16,
            virtual_height as u16 * dpi_factor as u16,
            conf::NumSamples::One, // As we don't need any anti-aliasing, for retro game looks
        )?;

        // Upscaling should be done with integral values.
        // Need to test this theory as a separate task, using canvas
        Ok(Push {
            canvas,
            scale: Vector2::new(
                screen_width / (virtual_width * dpi_factor),
                screen_height / (virtual_height * dpi_factor),
            ),
        })
    }

    pub fn start(&mut self, ctx: &mut Context) -> GameResult {
        self.canvas.set_filter(FilterMode::Nearest);
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::clear(ctx, graphics::Color::from_rgba(0, 0, 0, 0));
        Ok(())
    }

    pub fn end(&mut self, ctx: &mut Context) -> GameResult {
        graphics::set_canvas(ctx, None);

        graphics::draw(
            ctx,
            &self.canvas,
            graphics::DrawParam::default().scale(self.scale),
        )?;

        Ok(())
    }
}
