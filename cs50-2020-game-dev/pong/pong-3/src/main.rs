#![allow(dead_code, unused_imports)]

/**
 * This Chapter focuses on Printing just a Text, at the center of the Screen.
 */
use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{self, Color, DrawParam, FilterMode, Font, Scale, Text, TextFragment};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{conf, timer, filesystem, Context, ContextBuilder, GameResult};

use std::{env, path};

mod push;
mod player;
mod ball;
mod custom_text;

use player::{Player};
use ball::{Ball};
use custom_text::{CustomText, TextPosition};

pub const DESIRED_FPS: u32 = 60;
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

const VIRTUAL_WIDTH: f32 = 432.0;
const VIRTUAL_HEIGHT: f32 = 243.0;

const PADDLE_WIDTH: f32 = 5.0;
const PADDLE_HEIGHT: f32 = 20.0;
const BALL_RADIUS: f32 = 4.0;

/// Ggez doesn't work without state, we will be needing an initial state
/// that contains our Text mesh.
struct GameState {
    text: CustomText,
    player1_score_text: CustomText,
    player2_score_text: CustomText,
    push: push::Push,
    player1: Player,
    player2: Player,
    border: graphics::Mesh,
    ball: Ball,
    dpi_factor: f32,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;
        let text = CustomText::new(ctx, String::from("Hello Pong!"), None);
        let push = push::Push::new(
            ctx,
            VIRTUAL_WIDTH,
            VIRTUAL_HEIGHT,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )?;
        let player2_pos = Point2::new(
            WINDOW_WIDTH - (PADDLE_WIDTH * dpi_factor) - 10.0,
            WINDOW_HEIGHT - (PADDLE_HEIGHT * dpi_factor) - 30.0
        );
        let player1 = Player::new(
            ctx,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            Point2::new(10.0, 30.0),
            Some((KeyCode::W, KeyCode::S))
        )?;
        let player2 = Player::new(
            ctx,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            player2_pos,
            None
        )?;
        let player1_score_text = CustomText::new(ctx, player1.get_score_string(), Some(64.0));
        let player2_score_text = CustomText::new(ctx, player2.get_score_string(), Some(64.0));

        Ok(GameState {
            text,
            player1_score_text,
            player2_score_text,
            push,
            player1,
            player2,
            border: graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::Stroke(graphics::StrokeOptions::default()),
                graphics::Rect::new(1.0, 1.0, WINDOW_WIDTH - 2.0, WINDOW_HEIGHT - 2.0),
                Color::from_rgb(180, 100, 140),
            )?,
            ball: Ball::new(ctx, BALL_RADIUS, Point2::new(
                WINDOW_WIDTH / 2.0 - (BALL_RADIUS * dpi_factor),
                WINDOW_HEIGHT / 2.0 - (BALL_RADIUS * dpi_factor)
            ))?,
            dpi_factor,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let elapsed_time = 1.0 / DESIRED_FPS as f32;
            self.player1.update(ctx, elapsed_time)?;
            self.player2.update(ctx, elapsed_time)?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgba(40, 45, 52, 255));

        self.push.start(ctx)?;

        // Even though our canvas is of small virtual_size, but it still takes the actual window size
        // Somthing I guess ggez messes up internally. Thus, for any draw call, use the actual WINDOW_* Size

        // Later will move this printf ro `draw` calls, unlike the cs50 classes.
        self.text.printf(
            ctx, // I could've passed a string here instead, and never used a CustomText struct, but this is ok as well.
            0.0,
            20.0 * self.dpi_factor,
            Some(WINDOW_WIDTH),
            Some(TextPosition::CENTER),
        )?;

        self.player1_score_text.printf(
            ctx,
            WINDOW_WIDTH / 2.0 - 100.0,
            WINDOW_HEIGHT / 3.0,
            None,
            None
        )?;
        self.player2_score_text.printf(
            ctx,
            WINDOW_WIDTH / 2.0 + 50.0,
            WINDOW_HEIGHT / 3.0,
            None,
            None
        )?;

        // Every Draw calls, require to be multiplied by dpi_factor, ggez is confusing when comes to scaling
        self.player1.draw(ctx)?;
        self.player2.draw(ctx)?;
        self.ball.draw(ctx)?;

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
