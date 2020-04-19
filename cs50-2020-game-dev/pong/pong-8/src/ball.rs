use ggez::event::{EventHandler, KeyCode};
use ggez::graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh, Rect};
use ggez::{Context, GameResult};
use ggez::input::keyboard;
use ggez::nalgebra::{Point2};
use rand::Rng;

use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

use crate::player::{Player};

pub struct Ball {
    ball: Mesh,
    bbox: Rect,
    init_pos: Point2<f32>,
    pos: Point2<f32>,
    dx: f32,
    dy: f32,
}

impl Ball {
    // Point2::new(WINDOW_WIDTH / 2.0 - (2.0 * dpi_factor), WINDOW_HEIGHT / 2.0 - (2.0 * dpi_factor))
    pub fn new(ctx: &mut Context, radius: f32, init_pos: Point2<f32>) -> GameResult<Self> {
        let dpi_factor = graphics::window(ctx).get_hidpi_factor() as f32;
        let mut rng = rand::thread_rng();
        let dx = if rng.gen_range(1, 2) as u8 == 1 { 200.0 } else { -200.0 };
        let dy: f32 = rng.gen_range(-100.0, 100.0);
        let size = radius * dpi_factor;

        let ball = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::Fill(FillOptions::default()),
            Point2::new(0.0, 0.0),
            size,
            4.0,
            Color::from_rgb(255, 255, 255),
        )?;

        Ok(Ball {
            ball,
            bbox: Rect::new(0.0, 0.0, size, size),
            pos: init_pos,
            init_pos,
            dx,
            dy
        })
    }

    pub fn get_left_edge(&self) -> f32 {
        self.pos.x
    }
    pub fn get_right_edge(&self) -> f32 {
        self.pos.x + self.bbox.w
    }
    pub fn get_bottom_edge(&self) -> f32 {
        self.pos.y + self.bbox.h
    }
    pub fn get_top_edge(&self) -> f32 {
        self.pos.y
    }

    pub fn reset(&mut self) -> (f32, f32) {
        self.pos = self.init_pos;
        let mut rng = rand::thread_rng();
        let dx = if rng.gen_range(1, 3) as u8 == 1 { 200.0 } else { -200.0 };
        let dy: f32 = rng.gen_range(-100.0, 100.0) * 1.5;
        (dx, dy)
    }

    pub fn collides(&self, player: &Player) -> bool {
        if self.get_left_edge() > player.get_right_edge() || player.get_left_edge() > self.get_right_edge() {
            return false;
        }
        if self.get_top_edge() > player.get_bottom_edge() || player.get_top_edge() > self.get_bottom_edge() {
            return false;
        }

        return true;
    }

    pub fn reflect_with_offset(&mut self, offset: f32) {
        let mut rng = rand::thread_rng();
        self.dx = self.dx * -1.03;
        self.pos = Point2::new(self.pos.x + offset, self.pos.y);

        // Reflect to same direction in dy.
        if self.dy < 0.0 {
            self.dy = -rng.gen_range(10.0, 150.0);
        } else {
            self.dy = rng.gen_range(10.0, 150.0);
        }
    }

    pub fn update(&mut self, ctx: &mut Context, ellapsed_time: f32) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::Return) {
            let (dx, dy) = self.reset();
            self.dx = dx;
            self.dy = dy;
        } else {
            let new_x = self.pos.x + self.dx * ellapsed_time;
            let mut new_y = self.pos.y + self.dy * ellapsed_time;

            if new_y <= 0.0 {
                new_y = 0.0;
                self.dy = -self.dy;
            }
            if new_y >= WINDOW_HEIGHT - self.bbox.h {
                new_y = WINDOW_HEIGHT - self.bbox.h;
                self.dy = -self.dy;
            }

            self.pos = Point2::new(new_x, new_y);
        }
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.ball, DrawParam::default().dest(self.pos))?;
        Ok(())
    }
}
