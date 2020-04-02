/********************************************************************
 *
 * This sample code is to understand very basics or lifecycle methods
 * in ggez. It just shows, how to add a Background, with a moving
 * circle on it
 *
 *********************************************************************/

// Important Imports to have in any program using ggez
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

// Algebra can be replaced by any [MINT](https://docs.rs/mint/0.5.4/mint/index.html)
// spec based library, like nalgebra or something
// GGez just re-exports the nalgebra library for re-use.
use ggez::nalgebra as na;

// We need some initial state to start with, and update that state every frame loop,
// to get the desured output.
struct MainState {
    pos_x: f32,
}

// GameResult is nothing but a modified alias to Result<State, GameError>
// where, GameError is an enum, specific to various game related errors.
impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

// Following is event handler, that handles Draw cycles and Update Cycles, update cycles are
// called before Draw. Also, other event handlers like Keyboard or joystick input can be added
// in `key_down_event()` and so on.
impl event::EventHandler for MainState {

    /**
     * Before any draw, update the position in our state.
     */
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    /**
     * Once the update is done, use the state to draw whatever u want with that state.
     * This draw method is called every 16ms, to keep the framerate 60FPS.
     * The better the engine the more FPS we can get out of it. Not sure
     * if GGex supports more than 60FPS for now.
     */
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // This is the call any graphics library requires to do.
        // We need to clear our whole screen, to re-draw elements on our canvas.
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Create a Circle Mesh. Mesh is nothing but a shape, which can be 2D or 3D.
        // Here it's just simple 2D I guess.
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            100.0,
            // https://docs.rs/lyon_geom/0.15.2/lyon_geom/#flattening
            // TLDR: the less the tolerance is, the higher amount of segments, and more precise the circle.
            0.5,
            graphics::WHITE,
        )?;

        // Now draw the circle on our canvas, with new position.
        graphics::draw(ctx, &circle, (na::Point2::new(self.pos_x, 380.0),))?;

        // What I know of, present is called by vulkan backend to pass the command buffer to
        // to render cycle, which runs, all the above passed commands inside GPU.
        graphics::present(ctx)?;
        Ok(())
    }
}

// Our Apps, initial starting point.
pub fn main() -> GameResult {
    // This is a merge of `winit`, and `gfx-hal` contexts, that ggex simplifies as an API
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    // This I guess, internally calls `winit` build api, to build the window.
    // where ggez returns us the `winit` event_loop and newly created `ctx`
    // (that I guess is something ggez) specific
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new()?;

    // This will start the event_loop.
    event::run(ctx, event_loop, state)
}
