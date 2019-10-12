use std::time::{ Instant, Duration };
use ggez::{ GameResult, Context, graphics };
use ggez::event::{ KeyCode, KeyMods, EventHandler };
use super::constants::{ GRID_SIZE, MILLIS_PER_UPDATE };
use super::miscellaneous::{ GridPosition, Direction };
use super::snake::{ Snake, Ate };
use super::food::Food;

/// Now we have the heart of our game, the GameState. This struct
/// will implement ggez's `EventHandler` trait and will therefore drive
/// everything else that happens in our game.
pub struct GameState {
    /// First we need a Snake
    snake: Snake,
    /// A piee of food
    food: Food,
    /// Whether the game is over or not
    gameover: bool,
    /// And we track the last time we updated so that we can limit
    /// our update rate.
    last_update: Instant,
}

impl GameState {
    /// Our new function will set up the initial state of our game.
    pub fn new() -> Self {
        // First we put our snake a quarter of the way across our grid in the x axis
        // and half way down the y axis. This works well since we start out moving to the right.
        let snake_pos = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();
        // Then we choose a random place to put our piece of food using the helper we made
        // earlier.
        let food_pos = GridPosition::random(GRID_SIZE.0, GRID_SIZE.1);

        GameState {
            snake: Snake::new(snake_pos),
            food: Food::new(food_pos),
            gameover: false,
            last_update: Instant::now(),
        }
    }
}

/// Now we implement EventHandler for GameState. This provides an interface
/// that ggez will call automatically when different events happen.
impl EventHandler for GameState {
    /// Update will happen on every frame before it is drawn. This is where we update
    /// our game state to react to whatever is happening in the game world.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // First we check to see if enough time has elapsed since our last update based on
        // the update rate we defined at the top.
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            // Then we check to see if the game is over. If not, we'll update. If so, we'll just do nothing.
            if !self.gameover {
                // Here we do the actual updating of our game world. First we tell the snake to update itself,
                // passing in a reference to our piece of food.
                self.snake.update(&self.food);
                // Next we check if the snake ate anything as it updated.
                if let Some(ate) = self.snake.ate {
                    // If it did, we want to know what it ate.
                    match ate {
                        // If it ate a piece of food, we randomly select a new position for our piece of food
                        // and move it to this new position.
                        Ate::Food => {
                            let new_food_pos = GridPosition::random(GRID_SIZE.0, GRID_SIZE.1);
                            self.food.pos = new_food_pos;
                        }
                        // If it ate itself, we set our gameover state to true.
                        Ate::Itself => {
                            self.gameover = true;
                        }
                    }
                }
            }
            // If we updated, we set our last_update to be now
            self.last_update = Instant::now();
        }
        // Finally we return `Ok` to indicate we didn't run into any errors
        Ok(())
    }

    /// draw is where we should actually render the game's current state.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // First we clear the screen to a nice (well, maybe pretty glaring ;)) green
        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());
        // Then we tell the snake and the food to draw themselves
        self.snake.draw(ctx)?;
        self.food.draw(ctx)?;
        // Finally we call graphics::present to cycle the gpu's framebuffer and display
        // the new frame we just drew.
        graphics::present(ctx)?;
        // We yield the current thread until the next update
        ggez::timer::yield_now();
        // And return success.
        Ok(())
    }

    /// key_down_event gets fired when a key gets pressed.
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        // Here we attempt to convert the Keycode into a Direction using the helper
        // we defined earlier.
        if let Some(dir) = Direction::from_keycode(keycode) {
            // If it succeeds, we check if a new direction has already been set
            // and make sure the new direction is different then `snake.dir`
            if self.snake.dir != self.snake.last_update_dir && dir.inverse() != self.snake.dir {
                self.snake.next_dir = Some(dir);
            } else if dir.inverse() != self.snake.last_update_dir {
                // If no new direction has been set and the direction is not the inverse
                // of the `last_update_dir`, then set the snake's new direction to be the
                // direction the user pressed.
                self.snake.dir = dir;
            }
        }
    }
}
