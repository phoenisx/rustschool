/**
 * This code is taken from ggez examples - https://github.com/ggez/ggez/blob/master/examples/04_snake.rs
 * The code is modified to separate game modules for better understanding
 */

use ggez::{ GameResult, ContextBuilder, conf };
use ggez::event;
use random_walk_ggez::snake_game;

use snake_game::constants::SCREEN_SIZE;
use snake_game::world::GameState;

fn main() -> GameResult {
    // Here we use a ContextBuilder to setup metadata about our game. First the title and author
    let (ctx, events_loop) = &mut ContextBuilder::new("snake", "Snake Mysterio")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(conf::WindowSetup::default().title("Snake!"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build()?;

    // Next we create a new instance of our GameState struct, which implements EventHandler
    let state = &mut GameState::new();
    // And finally we actually run our game, passing in our context and state.
    event::run(ctx, events_loop, state)
}
