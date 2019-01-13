use ggez::graphics::Vector2;

pub struct Robot {
    pub position: Vector2,
    pub speed: Vector2
}

impl Robot {
    pub fn push(&mut self, force: &Vector2, time: f32){
        self.position += self.speed * time;
        self.speed += force * time;
    }
}

pub struct Round{
    pub center: Vector2,
    pub radius: f32,
}

impl Round {
    pub fn new(x: f32, y: f32, r:f32) -> Round {
        Round{center:Vector2::new(x,y), radius: r}
    }
}

pub struct World {
    pub rounds: Vec<Round>,
    pub robot: Robot
}

impl World {
    pub fn new(rounds: Vec<Round>) -> World {
        World{
            rounds: rounds,
            robot: Robot{
                position: Vector2::new(-1.0,-1.0),
                speed: Vector2::new(0.0, 0.0)
            }
        }
    }

    pub fn push_robot(&mut self, force: &Vector2, time: f32){
        self.robot.push(force, time);
    }

    pub fn check_collisions(&self) -> bool {
        for round in self.rounds.iter() {
            let r = round.center - self.robot.position;
            if r.x * r.x + r.y * r.y <= round.radius * round.radius {
                return false;
            }
        }
        return true;
    }

    pub fn check_borders(&self) -> bool {
        let (x, y) = (self.robot.position.x, self.robot.position.y);
        x >= -1.0 && x <= 1.0 && y >= -1.0 && y <= 1.0
    }

    pub fn bad_position(&self) -> bool {
        !self.check_borders() || !self.check_collisions()
    }
}
