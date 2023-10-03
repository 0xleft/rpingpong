use egui::{Pos2, Vec2};

pub struct Player {
    pub position: Pos2,
    pub velocity: Vec2,
    pub width: f32,
    pub height: f32,
    pub max_y: f32,
}

impl Player {
    pub fn new(position: Pos2, velocity: Vec2, width: f32, height: f32, max_y: f32, _: bool) -> Self {
        Self {
            position,
            velocity,
            width,
            height,
            max_y,
        }
    }

    pub fn update(&mut self) {
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

    pub fn add_velocity(&mut self, velocity: Vec2) {
        self.velocity += velocity;
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }
}

pub struct Ball {
    pub position: Pos2,
    pub velocity: Vec2,
    pub height: f32,
    pub max_y: f32,
    pub hits: u32,
}

impl Ball {
    pub fn new(position: Pos2, velocity: Vec2, _: f32, height: f32, max_y: f32) -> Self {
        Self {
            position,
            velocity,
            height,
            max_y,
            hits: 1,
        }
    }

    pub fn update(&mut self, player_positions: &Vec<Player>) {
        self.position += self.velocity;

        if self.position.y < 0.0  {
            self.velocity.y *= -1.0;
            self.position.y = 0.0;
        }
        if self.position.y + self.velocity.y + self.height > self.max_y  {
            self.velocity.y *= -1.0;
            self.position.y = self.max_y - self.height;
        }

        // left player
        if self.position.x < player_positions[0].position.x + player_positions[0].width {
            if self.position.y + self.height > player_positions[0].position.y && self.position.y < player_positions[0].position.y + player_positions[0].height {
                self.position.x = player_positions[0].position.x + player_positions[0].width;
                self.hits += 1;
                self.velocity.x *= -1.0;
            }
        }
        // right player
        if self.position.x + self.velocity.x + self.height > player_positions[1].position.x {
            if self.position.y + self.height > player_positions[1].position.y && self.position.y < player_positions[1].position.y + player_positions[1].height {
                self.position.x = player_positions[1].position.x - self.height;
                self.hits += 1;
                self.velocity.x *= -1.0;
            }
        }

        self.velocity *= 1.0 + 0.000001 * self.hits as f32;
    }

    pub fn add_velocity(&mut self, velocity: Vec2) {
        self.velocity += velocity;
    }
}