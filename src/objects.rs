use egui::{Pos2, Vec2};

pub trait GameObject {
    fn new(position: Pos2, velocity: Vec2, width: f32, height: f32, max_y: f32, is_round: bool) -> Self;
    fn update(&mut self);
    fn add_velocity(&mut self, velocity: Vec2);
    fn set_velocity(&mut self, velocity: Vec2);
}

pub struct Player {
    pub position: Pos2,
    pub velocity: Vec2,
    pub width: f32,
    pub height: f32,
    pub max_y: f32,
}

impl GameObject for Player {
    fn new(position: Pos2, velocity: Vec2, width: f32, height: f32, max_y: f32, _: bool) -> Self {
        Self {
            position,
            velocity,
            width,
            height,
            max_y,
        }
    }

    fn update(&mut self) {
        self.velocity *= 0.9;

        if self.position.y < 0.0  {
            self.velocity.y *= -0.4;
            self.position.y = 0.0;
        } else if self.position.y + self.velocity.y + self.height > self.max_y  {
            self.velocity.y *= -0.4;
            self.position.y = self.max_y - self.height;
        } else {
            self.position += self.velocity;
        }
    }

    fn add_velocity(&mut self, velocity: Vec2) {
        self.velocity += velocity;
    }

    fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }
}

pub struct Ball {
    pub position: Pos2,
    pub velocity: Vec2,
    pub height: f32,
    pub max_y: f32,
}

impl GameObject for Ball {
    fn new(position: Pos2, velocity: Vec2, _: f32, height: f32, max_y: f32, _: bool) -> Self {
        Self {
            position,
            velocity,
            height,
            max_y,
        }
    }

    fn update(&mut self) {
        self.position += self.velocity;

        if self.position.y < 0.0  {
            self.velocity.y *= -1.0;
            self.position.y = 0.0;
        } else if self.position.y + self.velocity.y + self.height > self.max_y  {
            self.velocity.y *= -1.0;
            self.position.y = self.max_y - self.height;
        }
    }

    fn add_velocity(&mut self, velocity: Vec2) {
        self.velocity += velocity;
    }

    fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }
}