extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::Fruit;
use crate::DIR;
use std::collections::LinkedList;

use opengl_graphics::GlGraphics;

use piston::input::*;

pub struct Snake {
    pub body: LinkedList<(i32, i32)>,
    pub dir: DIR,
}

impl Snake {
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square((x * 20) as f64, (y * 20) as f64, 20_f64))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(RED, square, transform, gl));
        });
    }

    pub fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();
        match self.dir {
            DIR::UP => new_head.1 -= 1,
            DIR::DOWN => new_head.1 += 1,
            DIR::LEFT => new_head.0 -= 1,
            DIR::RIGHT => new_head.0 += 1,
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
        self.check_boundary();
    }

    pub fn get_score(&self) -> usize {
        self.body.len()
    }

    pub fn check_boundary(&mut self) {
        self.body.iter_mut().for_each(|(x, y)| {
            if x < &mut 0 {
                *x += 35;
            } else if x > &mut 35 {
                *x -= 36;
            } else if y < &mut 0 {
                *y += 35;
            } else if y > &mut 35 {
                *y -= 36;
            }
        });
    }

    pub fn check_collision(&mut self, fruit: &mut Fruit) {
        let new_head = (*self.body.front().expect("Snake has no body")).clone();

        if fruit.pos_x == new_head.0 && fruit.pos_y == new_head.1 {
            fruit.redraw();
            self.body.push_front(new_head);
        }
    }
}
