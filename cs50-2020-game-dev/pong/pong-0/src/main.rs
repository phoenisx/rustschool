/**
 * This Chapter focuses on Printing just a Text, at the center of the Screen.
 */
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Text, TextFragment, Scale};
use ggez::nalgebra::Point2;
use ggez::{conf, Context, ContextBuilder, GameResult};

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

/// Following are custom made Text related data, that will help to mimic Love2D APIs
enum TextPosition {
    LEFT,
    CENTER,
    RIGHT,
}

struct CustomText {
    mesh: Text,
    width: u32,
    height: u32,
}

impl CustomText {
    fn new(ctx: &mut Context, string: String) -> Self {
        // Scale can be considered as Font Size, as scale takes a value in pixels.
        let fragment = TextFragment::new(string).scale(Scale::uniform(18.0));
        let mesh = Text::new(fragment);
        let (width, height) = mesh.dimensions(ctx);
        CustomText {
            mesh,
            width,
            height,
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
            TextPosition::CENTER => x = (width / 2.0) - (self.width as f32 / 2.0),
            TextPosition::RIGHT => x = width - self.width as f32,
            _ => {}
        }
        let params = DrawParam::default().dest(Point2::new(x, start_y));

        graphics::draw(ctx, &self.mesh, params)
    }
}

/// Ggez doesn't work without state, we will be needing an initial state
/// that contains our Text mesh.
struct GameState {
    text: CustomText,
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        let text = CustomText::new(ctx, String::from("Hello Pong!"));
        GameState { text }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0.3, 0.3, 0.3, 1.0));

        self.text.printf(
            ctx, // I could've passed a string here instead, and never used a CustomText struct, but this is ok as well.
            0.0,
            (WINDOW_HEIGHT / 2.0) - (self.text.height as f32 / 2.0),
            WINDOW_WIDTH,
            TextPosition::CENTER,
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("pong0", "Subroto")
        .window_setup(conf::WindowSetup::default().title("Pong 0"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    let state = &mut GameState::new(ctx);
    event::run(ctx, event_loop, state)
}
