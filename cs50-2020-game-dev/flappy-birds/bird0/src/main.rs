pub mod push;

use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color, DrawMode, DrawParam, FilterMode, Mesh, Rect};
use ggez::{conf, Context, ContextBuilder, GameResult};
use ggez::nalgebra::{Point2, Vector2};

use push::Push;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

const VIRTUAL_WIDTH: f32 = 432.0;
const VIRTUAL_HEIGHT: f32 = 243.0;

struct WorldState {
    ball: Mesh,
    bbox: Mesh,
    border: Mesh,
    canvas: Canvas,
    scale: Vector2<f32>,
}

impl WorldState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;
        let ball = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2::new(0.0, 0.0),
            4.0,
            1.0,
            Color::from_rgb(255, 255, 255),
        )?;
        let bbox = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            Rect::new(0.0, 0.0, 8.0, 8.0),
            Color::from_rgb(180, 100, 140),
        )?;
        let border = Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(2.0),
            Rect::new(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT),
            Color::from_rgb(120, 100, 180),
        )?;

        let canvas = Canvas::new(
            ctx,
            VIRTUAL_WIDTH as u16 * dpi_factor as u16,
            VIRTUAL_HEIGHT as u16 * dpi_factor as u16,
            conf::NumSamples::One, // As we don't need any anti-aliasing, for retro game looks
        )?;

        let scale = Vector2::new(
            WINDOW_WIDTH / (VIRTUAL_WIDTH * dpi_factor),
            WINDOW_HEIGHT / (VIRTUAL_HEIGHT * dpi_factor),
        );

        Ok(WorldState {
            ball,
            bbox,
            border,
            canvas,
            scale,
        })
    }
}

impl EventHandler for WorldState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgba(40, 45, 52, 255));

        self.canvas.set_filter(FilterMode::Nearest);
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::clear(ctx, graphics::Color::from_rgba(0, 0, 0, 0));

        graphics::draw(
            ctx,
            &self.ball,
            DrawParam::default()
                .dest(Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0))
        )?;
        graphics::draw(
            ctx,
            &self.bbox,
            DrawParam::default()
                .dest(Point2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0))
        )?;
        graphics::draw(
            ctx,
            &self.border,
            DrawParam::default()
                .dest(Point2::new(0.0, 0.0))
        )?;

        graphics::set_canvas(ctx, None);

        graphics::draw(
            ctx,
            &self.canvas,
            graphics::DrawParam::default().scale(self.scale),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("flappy_bird_0", "Subroto")
        .window_setup(conf::WindowSetup::default().title("Flappy Bird"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;

    let state = &mut WorldState::new(ctx)?;

    event::run(ctx, event_loop, state)
}
