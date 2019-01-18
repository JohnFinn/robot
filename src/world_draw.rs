extern crate ggez;
extern crate nalgebra;

type Vector2 = nalgebra::Vector2<f32>;

use ggez::{Context, GameResult, conf, ContextBuilder, event};
use ggez::graphics::{Point2, Drawable};
use ggez::event::EventHandler;
use ggez::graphics::{self, DrawMode, DrawParam};

use crate::world::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::thread;
use std::sync::mpsc;

pub enum Command{
    Init((Vec<Round>, Vector2)),
    MoveRobot(Vector2)
}

pub struct WorldRenderer{
    handle: thread::JoinHandle<GameResult<()>>,
    transmitter: mpsc::Sender<Command>
}

impl WorldRenderer{
    pub fn new() -> WorldRenderer {
        let cb = ContextBuilder::new("robot", "jouny")
            .window_setup(conf::WindowSetup::default().title("robot"))
            .window_mode(conf::WindowMode::default().dimensions(1000, 1000));

        let (transmitter, receiver) = mpsc::channel();
        let handle = thread::spawn(move || {
            let mut context = cb.build().unwrap();
            let mut state = WorldState::new(&mut context, receiver);
            event::run(&mut context, &mut state)
        });
        WorldRenderer{handle: handle, transmitter}
    }

    pub fn init_with(&self, world: &World) -> Result<(), mpsc::SendError<Command>>{
        self.transmitter.send(Command::Init((world.rounds.clone(), world.robot.borrow().position.clone())))
    }

    pub fn move_robot(&self, world: &World) -> Result<(), mpsc::SendError<Command>>{
        self.transmitter.send(Command::MoveRobot(world.robot.borrow().position.clone()))
    }
}

struct WorldState {
    rounds: Vec<graphics::Mesh>,
    robot: graphics::Mesh,
    receiver: mpsc::Receiver<Command>
}

fn new_robot(context: &mut Context, center: &Vector2, radius: f32) -> GameResult<graphics::Mesh> {
    let center = to_screen_coordinates(context, center);
    let radiuss = to_screen_distanse(context, radius);
    graphics::Mesh::new_circle(context, DrawMode::Fill, center, radiuss, 0.1)
}

impl WorldState {
    fn new(context: &mut Context, receiver: mpsc::Receiver<Command>) -> WorldState {
        let robot_mesh = new_robot(context, &Vector2::new(-1.0, -1.0), 0.01).unwrap();
        WorldState{rounds: Vec::new(), robot: robot_mesh, receiver}
    }

    fn init(&mut self, rounds: &Vec<Round>, robot: &Vector2, context: &mut Context) {
        self.rounds.clear();
        self.rounds.reserve(rounds.len());
        for r in rounds.iter() {
            let center = to_screen_coordinates(context, &r.center);
            let radius = to_screen_distanse(context, r.radius);
            let mesh = graphics::Mesh::new_circle(context, DrawMode::Fill, center, radius, 0.1).unwrap();
            self.rounds.push(mesh);
        }
        self.move_robot(robot, context);
    }

    fn move_robot(&mut self, position: &Vector2, context: &mut Context){
        self.robot = new_robot(context, position, 0.01).unwrap();
    }
}

impl EventHandler for WorldState {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        match self.receiver.recv_timeout(std::time::Duration::from_millis(1000)) {
            Err(_) => Ok(()),
            Ok(command) => match command {
                Command::Init((rounds, robot)) => {
                    self.init(&rounds, &robot, context);
                    Ok(())
                },
                Command::MoveRobot(position) => {
                    self.move_robot(&position, context);
                    Ok(())
                }
            }
        }
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context);
        graphics::set_background_color(context, (255, 255, 255, 0).into());
        graphics::set_color(context, graphics::BLACK)?;
        for round in self.rounds.iter() {
            round.draw(context, Point2::new(0.0, 0.0), 0.0)?;
        }
        graphics::set_color(context, (255, 0, 0).into())?;
        self.robot.draw(context, Point2::new(0.0, 0.0), 0.0)?;
        graphics::present(context);
        Ok(())
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
