extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::Fruit;
use crate::Snake;
use crate::DIR;
use graphics::Transformed;
use opengl_graphics::GlyphCache;
use opengl_graphics::TextureSettings;

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
        // self.draw_text(arg);
        println!("Score: {}", self.snake.get_score());
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

    pub fn draw_text(&mut self, arg: &RenderArgs) {
        let font =
            "C:/Users/mstan/Documents/Fonts/FiraCode/'Fura Code Regular Nerd Font Complete.ttf'";
        let mut glyphs = GlyphCache::new(font, (), TextureSettings::new()).unwrap();
        self.gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform.trans(10.0, 100.0);
            const BLACK: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            const SIZE: u32 = 32;
            graphics::text::Text::new_color(BLACK, SIZE)
                .draw("Hello world!", &mut glyphs, &c.draw_state, transform, gl)
                .unwrap();
        });
    }
}
