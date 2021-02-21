extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::Fruit;
use crate::Snake;
use crate::DIR;

use opengl_graphics::GlGraphics;

use piston::input::*;

pub struct Game {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub fruit: Fruit,
}

impl Game {
    pub fn render(&mut self, arg: &RenderArgs) {
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });
        self.snake.render(&mut self.gl, arg);
        self.fruit.render(&mut self.gl, arg);
    }

    pub fn update(&mut self) {
        self.snake.update();
        self.snake.check_collision(&mut self.fruit);
    }

    pub fn pressed(&mut self, btn: &Button) {
        let last_dir = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_dir != DIR::DOWN => DIR::UP,
            &Button::Keyboard(Key::Down) if last_dir != DIR::UP => DIR::DOWN,
            &Button::Keyboard(Key::Right) if last_dir != DIR::LEFT => DIR::RIGHT,
            &Button::Keyboard(Key::Left) if last_dir != DIR::RIGHT => DIR::LEFT,
            _ => last_dir,
        }
    }
}
