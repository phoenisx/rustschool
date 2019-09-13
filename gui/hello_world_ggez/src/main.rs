// Learn from https://github.com/ggez/ggez/blob/master/docs/guides/HelloGgez.md
// But the code is modified to work with actual graphics...

use ggez::*;

struct GameState {
    dt: std::time::Duration,
    g_text: graphics::Text,
}

impl GameState {
    fn new() -> Self {
        GameState {
            dt: std::time::Duration::new(0, 0),
            g_text: graphics::Text::new("Hello World"),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgba(50, 50, 50, 255));

        // Not sure how efficient, setting the text on each render loop, is???
        self.g_text = graphics::Text::new(format!("Hello World: {}ms", self.dt.subsec_millis()));
        let dest_point = nalgebra::Point2::new(2.0, 10.0);
        graphics::draw(ctx, &self.g_text, graphics::DrawParam::default().dest(dest_point))?;
        graphics::present(ctx)?;
        // println!("Hello ggez! dt = {}ns", self.dt.subsec_nanos());
        Ok(())
    }
}

pub fn main() {
    let mut state = GameState::new();

    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "subroto")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, &mut state).unwrap();
}
