// import lib
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
//use piston::Window;
use std::collections::LinkedList;

use std::iter::FromIterator;

#[derive(Clone, PartialEq)]
enum Direction{
    Right, Left, Up, Down
}

struct Game {
    gl: GlGraphics,
    snake: Snake
}

impl Game{
    fn render(&mut self, arg: &RenderArgs){
        use graphics;

        let GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl|{
            graphics::clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, arg);
    }

    fn update(&mut self){
        self.snake.update();
    }

    fn pressed(&mut self, btn: & Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key:: Up)
                if last_direction != Direction::Down =>Direction::Up,
            &Button::Keyboard(Key:: Down)
            if last_direction != Direction::Up =>Direction::Down,
            &Button::Keyboard(Key:: Left)
            if last_direction != Direction::Right =>Direction::Left,
            &Button::Keyboard(Key:: Right)
            if last_direction != Direction::Left =>Direction::Right,
            _=> last_direction
        };
    }
}


struct Snake {
    //
    body: LinkedList<(i32, i32)>,
    dir: Direction,
}

impl Snake {
    fn render (&self, gl: &mut GlGraphics, args: &RenderArgs) {

        use graphics;

        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)|
                {
                graphics::rectangle::square(
                    (self.pos_x * 20) as f64,
                    (self.pos_y * 20) as f64,
                    20_f64);
                 })
            .collect();




        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares.into_iter()
                .for_each(|square|graphics::rectangle(RED, square, transform, gl));


        });
    }

    fn update(&mut self)
    {
        let mut new_head = (*self.body.front().expect("Snake has no body ")).clone();

        match self.dir {
            Direction::Left => new_head.0 -=1,
            Direction::Right  => new_head.0 +=1,
            Direction::Up => new_head.1 -=1,
            Direction::Down => new_head.1 +=1,
        }

        self.body.push_front(new_head);

        self.body.pop_back().unwrap();
    }
}


fn main() {

    let opengl = OpenGL::V4_4;

    let mut window:  GlutinWindow = WindowSettings::new(
        "Snake Game",
        [200, 200]
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0,0), (0, 1)]).into_iter()),
            dir: Direction::Right},
        //rotation: 0.0;
    };

    let mut events = Events::new(EventSettings::new()).ups(8);

    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args(){
            game.update();
        }
    }
}