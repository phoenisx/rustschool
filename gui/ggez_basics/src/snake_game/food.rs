use ggez::{ GameResult, Context, graphics };
use super::miscellaneous::GridPosition;

/// This is again an abstraction over a `GridPosition` that represents
/// a piece of food the snake can eat. It can draw itself.
pub struct Food {
    pub pos: GridPosition,
}

impl Food {
    pub fn new(pos: GridPosition) -> Self {
        Food { pos }
    }

    /// Here is the first time we see what drawing looks like with ggez.
    /// We have a function that takes in a `&mut ggez::Context` which we use
    /// with the helpers in `ggez::graphics` to do drawing. We also return a
    /// `ggez::GameResult` so that we can use the `?` operator to bubble up
    /// failure of drawing.
    ///
    /// Note: this method of drawing does not scale. If you need to render
    /// a large number of shapes, use a SpriteBatch. This approach is fine for
    /// this example since there are a fairly limited number of calls.
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // First we set the color to draw with, in this case all food will be
        // colored blue.
        let color = [0.0, 0.0, 1.0, 1.0].into();
        // Then we draw a rectangle with the Fill draw mode, and we convert the
        // Food's position into a `ggez::Rect` using `.into()` which we can do
        // since we implemented `From<GridPosition>` for `Rect` earlier.
        let rectangle =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.pos.into(), color)?;
        graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))
    }
}
