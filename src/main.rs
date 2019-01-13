extern crate ggez;

use ggez::{Context, ContextBuilder, GameError, conf, event, GameResult};
use ggez::graphics::{Point2,Vector2};
use ggez::event::EventHandler;
use ggez::graphics::{self,Mesh,DrawMode};

mod world;
use self::world::*;

impl EventHandler for World {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context);
        graphics::set_background_color(context, (255, 255, 255, 0).into());
        graphics::set_color(context, graphics::BLACK)?;
        for round in self.rounds.iter() {
            let center = to_screen_coordinates(context, &round.center);
            let radius = to_screen_distanse(context, round.radius);
            graphics::circle(context, DrawMode::Fill, center, radius, 0.1)?;
        }
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
    distance * context.conf.window_mode.width as f32
}

fn main() {
    let mut cb = ContextBuilder::new("robot", "jouny")
        .window_setup(conf::WindowSetup::default().title("robot"))
        .window_mode(conf::WindowMode::default().dimensions(1000, 1000));
    let context = &mut cb.build().unwrap();
    let rounds = vec![
        Round::new(1.0, 1.0, 0.1),
        Round::new(-1.0, 0.0, 0.05),
        Round::new(1.0, -1.0, 0.025),
    ];
    let mut state = World::new(rounds);
    if let Err(e) = event::run(context, &mut state) {
        println!("Error encountered running game: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
