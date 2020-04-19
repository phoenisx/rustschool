use ggez::graphics::{self, DrawParam, Font, Scale, Text, TextFragment};
use ggez::{Context, GameResult};
use ggez::nalgebra::{Point2, Vector2};

/// Following are custom made Text related data, that will help to mimic Love2D APIs
pub enum TextPosition {
    LEFT,
    CENTER,
    RIGHT,
}

pub struct CustomText {
    mesh: Text,
    width: u32,
    height: u32,
    scale: Vector2<f32>,
    dpi_factor: f32,
}

impl CustomText {
    pub fn new(ctx: &mut Context, string: String, size: Option<f32>) -> Self {
        // Getting crisp text using this logic: https://github.com/ggez/ggez/issues/561
        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;

        // Scale can be considered as Font Size, as scale takes a value in pixels.
        let fragment = if let Some(_size) = size {
            TextFragment::new(string)
                .font(Font::new(ctx, "/fonts/font.ttf").unwrap())
                .scale(Scale::uniform(_size * dpi_factor))
        } else {
            TextFragment::new(string)
                .font(Font::new(ctx, "/fonts/font.ttf").unwrap())
                .scale(Scale::uniform(24.0 * dpi_factor))
        };

        let mesh = Text::new(fragment);
        let (width, height) = mesh.dimensions(ctx);
        let scale = 1.0 / dpi_factor;
        CustomText {
            mesh,
            width,
            height,
            scale: Vector2::new(scale, scale),
            dpi_factor,
        }
    }

    // This is a simple replication of Love2D example shown for this Class.
    pub fn printf(
        &mut self,
        ctx: &mut Context,
        start_x: f32,
        start_y: f32,
        width: f32,
        position: TextPosition,
    ) -> GameResult {
        let mut x = start_x;
        match position {
            TextPosition::CENTER => {
                x = (width / 2.0) - (self.width as f32 / (2.0 * self.dpi_factor))
            }
            TextPosition::RIGHT => x = width - self.width as f32,
            _ => {}
        }


        let params = DrawParam::default()
            .dest(Point2::new(x, start_y))
            .scale(self.scale);

        graphics::draw(ctx, &self.mesh, params)
    }
}
