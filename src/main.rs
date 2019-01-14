extern crate ggez;
extern crate serde;
extern crate serde_json;
extern crate rand;

#[macro_use]
extern crate serde_derive;

use ggez::{Context, ContextBuilder, conf, event, GameResult};
use ggez::graphics::{Point2, Vector2, Drawable};
use ggez::event::EventHandler;
use ggez::graphics::{self, DrawMode, DrawParam};

use rand::Rng;

use std::fs::File;
use std::io::Read;

mod world;
use self::world::*;

mod pilots;
use self::pilots::*;

impl<'a> EventHandler for World<'a> {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        match self.tick(){
            Ok(_) => (),
            Err(_) => {
                let mut robot = self.robot.borrow_mut();
                robot.position = Vector2::new(-1.0, -1.0);
                robot.speed = Vector2::new(0.0, 0.0);
            }
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context);
        graphics::set_background_color(context, (255, 255, 255, 0).into());
        graphics::set_color(context, graphics::BLACK)?;
        for round in self.rounds.iter() {
            round.draw(context, Point2::new(0.0, 0.0), 0.0)?;
        }
        graphics::set_color(context, (255, 0, 0).into())?;
        self.robot.borrow().draw(context, Point2::new(0.0, 0.0), 0.0)?;
        graphics::present(context);
        Ok(())
    }
}

impl Drawable for Round {
    fn draw_ex(&self, context: &mut Context, _param: DrawParam) -> GameResult<()> {
        let center = to_screen_coordinates(context, &self.center);
        let radius = to_screen_distanse(context, self.radius);
        graphics::circle(context, DrawMode::Fill, center, radius, 0.1)?;
        Ok(())
    }

    fn set_blend_mode(&mut self, _mode: Option<graphics::BlendMode>){

    }

    fn get_blend_mode(&self) -> Option<graphics::BlendMode> {
        None
    }
}

impl Drawable for Robot {
    fn draw_ex(&self, context: &mut Context, _param: DrawParam) -> GameResult<()> {
        let center = to_screen_coordinates(context, &self.position);
        let radius = to_screen_distanse(context, 0.01);
        graphics::polygon(context, DrawMode::Fill, &[center, center])?;
        graphics::circle(context, DrawMode::Fill, center, radius, 0.1)?;
        Ok(())
    }

    fn set_blend_mode(&mut self, _mode: Option<graphics::BlendMode>){

    }

    fn get_blend_mode(&self) -> Option<graphics::BlendMode> {
        None
    }
}

fn to_screen_coordinates(context: &Context, point: &Vector2) -> Point2 {
    let x = (1.0 + point.x) * context.conf.window_mode.width as f32 / 2.0;
    let y = (1.0 - point.y) * context.conf.window_mode.height as f32 / 2.0;
    Point2::new(x, y)
}

fn to_screen_distanse(context: &Context, distance: f32) -> f32 {
    distance * context.conf.window_mode.width as f32 / 2.0
}

#[derive(Serialize, Deserialize, Debug)]
struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

impl std::convert::From<&Circle> for Round {
    fn from(circle: &Circle) -> Round {
        Round::new(circle.x, circle.y, circle.r)
    }
}

fn random_rounds(amt: usize) -> Vec<Round> {
    let mut gen = rand::thread_rng();
    (0..amt).map(|_| Round::new(
        gen.gen_range(-1.0, 1.0),
        gen.gen_range(-1.0, 1.0),
        gen.gen_range(0.01, 0.3)
    )).collect()
}

fn main() {
    /*
    let mut file = File::open("inputs.json").unwrap();
    let mut rounds : String = String::new();
    file.read_to_string(&mut rounds).unwrap();
    let rounds : Vec<Circle> = serde_json::from_str(&rounds).unwrap();
    let rounds: Vec<Round> = rounds.iter()
        .map(Round::from)
        .collect();
    */
    let x = Vector2::new(1.0, 1.0);
    let y = Vector2::new(1.0, 0.0);
    println!("{}", ggez::nalgebra::dot(&x, &y));
    println!("{}", ggez::nalgebra::dot(&y, &x));
    let rounds = vec![
        Round::new(0.8, 0.7, 0.01)
    ];

    let mut cb = ContextBuilder::new("robot", "jouny")
        .window_setup(conf::WindowSetup::default().title("robot"))
        .window_mode(conf::WindowMode::default().dimensions(1000, 1000));
    let context = &mut cb.build().unwrap();

    let pilot = Pilot1::new();
    let mut state = World::new(rounds, &pilot, 0.001, 1.0);

    if let Err(e) = event::run(context, &mut state) {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
