use crate::RenderArgs;
use opengl_graphics::GlGraphics;
use rand::Rng;

pub struct Fruit {
    pub pos_x: i32,
    pub pos_y: i32,
}

impl Fruit {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square =
            graphics::rectangle::square((self.pos_x * 20) as f64, (self.pos_y * 20) as f64, 20_f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(BLUE, square, transform, gl);
        });
    }

    pub fn redraw(&mut self) {
        self.pos_x = rand::thread_rng().gen_range(0..35);
        self.pos_y = rand::thread_rng().gen_range(0..35);
    }
}
