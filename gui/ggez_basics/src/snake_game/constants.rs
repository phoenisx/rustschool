// Here we define the size of our game board in terms of how many grid
// cells it will take up. We choose to make a 30 x 20 game board.
pub const GRID_SIZE: (i16, i16) = (30, 20);
// Now we define the pixel size of each tile, which we make 32x32 pixels.
pub const GRID_CELL_SIZE: (i16, i16) = (32, 32);

// Next we define how large we want our actual window to be by multiplying
// the components of our grid size by its corresponding pixel size.
pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

// Here we're defining how many quickly we want our game to update. This will be
// important later so that we don't have our snake fly across the screen because
// it's moving a full tile every frame.
pub const UPDATES_PER_SECOND: f32 = 8.0;
// And we get the milliseconds of delay that this update rate corresponds to.
pub const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;
