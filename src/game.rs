use egui::{Vec2, Pos2, epaint::CircleShape};

use crate::objects::{self, GameObject};
use egui::{containers::*, *};

pub struct PingPong {
    pub players: Vec<objects::Player>,
    pub score: (u32, u32),
    pub ball: objects::Ball,
    pub playing: bool,
    pub last_winner: u32,
}

impl PingPong {
    pub fn new(screen_size: Vec2) -> Self {
        let mut players = Vec::new();

        let player1 = objects::Player::new(
            Pos2::new(0.0, screen_size.y / 2.0),
            Vec2::new(0.0, 0.0),
            10.0,
            80.0,
            screen_size.y,
            false,
        );

        let player2 = objects::Player::new(
            Pos2::new(screen_size.x - 10.0, screen_size.y / 2.0),
            Vec2::new(0.0, 0.0),
            10.0,
            80.0,
            screen_size.y,
            false,
        );

        players.push(player1);
        players.push(player2);

        let score = (0, 0);

        let ball = objects::Ball::new(
            Pos2::new(screen_size.x / 2.0, screen_size.y / 2.0),
            Vec2::new(0.0, 0.0),
            10.0,
            10.0,
            screen_size.y,
            true,
        );

        Self {
            players,
            score,
            ball,
            playing: false,
            last_winner: 0,
        }
    }
}

impl eframe::App for PingPong {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(format!("Player 1: {}", self.score.0)).font(FontId::proportional(30.0)));
                ui.label(RichText::new(format!("Player 2: {}", self.score.1)).font(FontId::proportional(30.0)));
            });

            egui::containers::Frame::canvas(ui.style()).show(ui, |ui| {
                // ui.ctx().request_repain_after(Duration::from_millis(1));

                if ctx.input(|i| i.key_down(Key::W)) {
                    self.players[0].add_velocity(Vec2::new(0.0, -1.0));
                }

                if ctx.input(|i| i.key_down(Key::S)) {
                    self.players[0].add_velocity(Vec2::new(0.0, 1.0));
                }

                if ctx.input(|i| i.key_down(Key::ArrowDown)) {
                    self.players[1].add_velocity(Vec2::new(0.0, 1.0));
                }

                if ctx.input(|i| i.key_down(Key::ArrowUp)) {
                    self.players[1].add_velocity(Vec2::new(0.0, -1.0));
                }

                for player in &mut self.players {
                    player.update();

                    let position = player.position;
                    
                    let size = vec2(player.width, player.height);
                    let rect = Rect::from_min_size(position, size);
                    ui.painter().rect_filled(rect, 0.0, Color32::WHITE);
                }

                self.ball.update();
                let position = self.ball.position;
                let size = vec2(self.ball.height, self.ball.height);
                let oval = CircleShape {
                    center: position,
                    radius: size.x / 2.0,
                    fill: Color32::WHITE,
                    stroke: Stroke::new(1.0, Color32::BLACK),
                };
                ui.painter().add(Shape::Circle(oval));

                ui.ctx().request_repaint();
            });
        });
    }
}