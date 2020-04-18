#![allow(dead_code, unused_imports)]

/**
 * This Chapter focuses on Printing just a Text, at the center of the Screen.
 */
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, FilterMode, Font, Scale, Text, TextFragment};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{conf, filesystem, Context, ContextBuilder, GameResult};

use std::{env, path};

mod push;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

const VIRTUAL_WIDTH: f32 = 432.0;
const VIRTUAL_HEIGHT: f32 = 243.0;

/// Following are custom made Text related data, that will help to mimic Love2D APIs
/* #region */
enum TextPosition {
    LEFT,
    CENTER,
    RIGHT,
}

struct CustomText {
    mesh: Text,
    width: u32,
    height: u32,
    dpi_factor: f32,
}

impl CustomText {
    fn new(ctx: &mut Context, string: String) -> Self {
        // Getting crisp text using this logic: https://github.com/ggez/ggez/issues/561
        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;

        // Scale can be considered as Font Size, as scale takes a value in pixels.
        let fragment = TextFragment::new(string)
            .font(Font::new(ctx, "/fonts/font.ttf").unwrap())
            .scale(Scale::uniform(24.0 * dpi_factor)); // Take a high resolution Font, and will later scale it down,
                                                       // as for now, text does not have any interpolation built in.
        let mesh = Text::new(fragment);
        let (width, height) = mesh.dimensions(ctx);
        CustomText {
            mesh,
            width,
            height,
            dpi_factor,
        }
    }

    // This is a simple replication of Love2D example shown for this Class.
    fn printf(
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

        let scale = 1.0 / self.dpi_factor;
        let params = DrawParam::default()
            .dest(Point2::new(x, start_y))
            .scale(Vector2::new(scale, scale));

        graphics::draw(ctx, &self.mesh, params)
    }
}
/* #endregion End Custom Text */

/// Ggez doesn't work without state, we will be needing an initial state
/// that contains our Text mesh.
struct GameState {
    text: CustomText,
    push: push::Push,
    border: graphics::Mesh,
    paddle_rect: graphics::Mesh,
    ball_circle: graphics::Mesh,
    dpi_factor: f32,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;
        let text = CustomText::new(ctx, String::from("Hello Pong!"));
        let push = push::Push::new(
            ctx,
            VIRTUAL_WIDTH,
            VIRTUAL_HEIGHT,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )?;
        Ok(GameState {
            text,
            push,
            border: graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::Stroke(graphics::StrokeOptions::default()),
                graphics::Rect::new(1.0, 1.0, WINDOW_WIDTH - 2.0, WINDOW_HEIGHT - 2.0),
                Color::from_rgb(180, 100, 140),
            )?,
            paddle_rect: graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::Fill(graphics::FillOptions::default()),
                graphics::Rect::new(10.0, 30.0, 10.0, 40.0),
                Color::from_rgb(255, 255, 255),
            )?,
            ball_circle: graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::Fill(graphics::FillOptions::default()),
                Point2::new(WINDOW_WIDTH / 2.0 - (2.0 * dpi_factor), WINDOW_HEIGHT / 2.0 - (2.0 * dpi_factor)),
                4.0 * dpi_factor,
                3.0,
                Color::from_rgb(255, 255, 255),
            )?,
            dpi_factor,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgba(40, 45, 52, 255));

        self.push.start(ctx)?;

        // Even though our canvas is of small virtual_size, but it still takes the actual window size
        // Somthing I guess ggez messes up internally. Thus, for any draw call, use the actual WINDOW_* Size

        self.text.printf(
            ctx, // I could've passed a string here instead, and never used a CustomText struct, but this is ok as well.
            0.0,
            20.0 * self.dpi_factor,
            WINDOW_WIDTH,
            TextPosition::CENTER,
        )?;

        // Every Draw calls, require to be multiplied by dpi_factor, ggez is confusing when comes to scaling
        graphics::draw(ctx, &self.paddle_rect, DrawParam::default())?;
        graphics::draw(
            ctx,
            &self.paddle_rect,
            DrawParam::default().dest(Point2::new(WINDOW_WIDTH - 30.0, WINDOW_HEIGHT - 110.0)),
        )?;
        graphics::draw(
            ctx,
            &self.ball_circle,
            DrawParam::default(),
        )?;

        graphics::draw(ctx, &self.border, DrawParam::default())?;

        self.push.end(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("../resources") // It is important to name the folder `resources`
    };

    let (ctx, event_loop) = &mut ContextBuilder::new("pong0", "Subroto")
        .window_setup(conf::WindowSetup::default().title("Pong 0"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .add_resource_path(resource_dir)
        .build()?;
    let state = &mut GameState::new(ctx)?;

    // filesystem::print_all(ctx);

    event::run(ctx, event_loop, state)
}
