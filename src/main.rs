extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::piston::window::AdvancedWindow;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use std::iter::FromIterator;

use rand::Rng;
use std::collections::LinkedList;

mod fruit;
mod game;
mod snake;

use fruit::Fruit;
use game::Game;
use snake::Snake;

#[derive(Clone, PartialEq)]
pub enum DIR {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Snake", [700, 700])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1)]).into_iter()),
            dir: DIR::RIGHT,
        },
        fruit: Fruit {
            pos_x: rand::thread_rng().gen_range(0..35),
            pos_y: rand::thread_rng().gen_range(0..35),
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }
        if let Some(_u) = e.update_args() {
            game.update();
            window.set_title(format!("Snake Game | Score: {}", game.snake.get_score()));
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
