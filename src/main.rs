#![allow(clippy::unnecessary_wraps)]
#![windows_subsystem = "windows"]

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::event::{KeyCode, KeyMods};
use glam::*;
use ggez::timer;
use ggez_egui::EguiBackend;

mod ball;
use ball::Ball;

mod paddle;
use paddle::Paddle;

mod ai;
use ai::AIPlayer;

mod player;
use player::Player;

mod gameplay;
use gameplay::Side;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;

const GUI_MIN: egui::Pos2 = egui::pos2(0.0, WINDOW_HEIGHT);
const GUI_MAX: egui::Pos2 = egui::pos2(WINDOW_WIDTH, WINDOW_HEIGHT + 100.0);

struct GameState
{
    egui_backend: EguiBackend,
    state: i32,
    ball: Ball,
    player: Player,
    ai: AIPlayer,
}

impl GameState 
{
    fn new(ctx: &mut Context) -> GameResult<GameState> 
    {
        let ball = Ball::new(ctx, WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0, 20.0);
        let paddle1 = Paddle::new(ctx, 30.0, WINDOW_HEIGHT / 2.0, 30.0, 150.0, WINDOW_HEIGHT);
        let player = Player::new(paddle1, 400.0);
        let paddle2 = Paddle::new(ctx, WINDOW_WIDTH - 30.0, WINDOW_HEIGHT / 2.0, 30.0, 150.0, WINDOW_HEIGHT);
        let ai = AIPlayer::new(paddle2, 400.0);

        let s = GameState 
        { 
            egui_backend: EguiBackend::default(),
            state: 0,
            ball: ball,
            player: player,
            ai: ai,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GameState
{
    fn update(&mut self, ctx: &mut Context) -> GameResult 
    {
        //egui logic
        let egui_ctx = self.egui_backend.ctx(); 
        egui::Window::new("")
            .fixed_rect(egui::Rect {min: GUI_MIN, max: GUI_MAX})
            .collapsible(false)
            .title_bar(false)
            .show(&egui_ctx, |ui|
            {
                //score labels
                let score_player = egui::Label::new(format!("Player\n{}", self.player.score))
                                                .text_color(egui::Color32::YELLOW)
                                                .text_style(egui::TextStyle::Heading);
                ui.put(egui::Rect::from_center_size(egui::pos2(50.0, 840.0), egui::vec2(100.0, 20.0)), score_player);
                let score_ai = egui::Label::new(format!("Computer\n{}", self.ai.score))
                                            .text_color(egui::Color32::YELLOW)
                                            .text_style(egui::TextStyle::Heading);
                ui.put(egui::Rect::from_center_size(egui::pos2(WINDOW_WIDTH - 50.0, 840.0), egui::vec2(100.0, 20.0)), score_ai);

                //help text
                if self.state == 0
                {
                    let instructions = egui::Label::new("Press SPACE to start")
                                                    .text_color(egui::Color32::WHITE)
                                                    .text_style(egui::TextStyle::Heading);
                    ui.put(egui::Rect::from_center_size(egui::pos2(WINDOW_WIDTH / 2.0, 850.0), egui::vec2(200.0, 50.0)), instructions);
                }

                //fill the empty space
                ui.allocate_space(ui.available_size());
            });
        /*
        egui::Window::new("Score window").show(&egui_ctx, |ui| 
            {
                ui.heading("ASS EATING");
                if ui.button("ass").clicked()
                {
                    println!("ass eaten");
                }
            }); */



        //game logic
        let dt = timer::delta(ctx);
        //if game is running
        if self.state == 1
        {
            self.ball.update(ctx, dt)?;
            self.player.update(ctx, dt)?;
            self.ai.update(ctx, dt, &self.ball)?;

            gameplay::collision_detection(&mut self.ball, &self.player.paddle, &self.ai.paddle, WINDOW_WIDTH, WINDOW_HEIGHT);
            let miss = gameplay::miss_detection(&self.ball, WINDOW_WIDTH);
            if miss.is_some()
            {
                let miss = miss.unwrap();
                match miss
                {
                    Side::Left => { self.ai.score += 1; }
                    Side::Right => { self.player.score += 1; }
                };

                self.ball.reset();
                self.ai.paddle.reset();
                self.player.paddle.reset();
                self.state = 0;
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult 
    {
        graphics::clear(ctx, Color::BLUE);

        //draw egui
        graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;

        //draw game
        self.ball.draw(ctx)?;
        self.player.draw(ctx)?;
        self.ai.draw(ctx)?;
        
        graphics::present(ctx)?;
        Ok(())
    }
    
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) 
    {
        if self.state == 0
        {
            if keycode == KeyCode::Space
            {
                self.ball.ping();
                self.state = 1;
            }
        }
        else if self.state == 1
        {
            if keycode == KeyCode::Up || keycode == KeyCode::W
            {
                self.player.paddle.move_up(self.player.max_speed);
            }
            else if keycode == KeyCode::Down || keycode == KeyCode::S
            {
                self.player.paddle.move_down(self.player.max_speed);
            }
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods)
    {
        if self.state == 1
        {
            if keycode == KeyCode::Up || keycode == KeyCode::W || keycode == KeyCode::Down || keycode == KeyCode::S
            {
                self.player.paddle.stop();
            }        
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: event::MouseButton, _x: f32, _y: f32)
    {
        self.egui_backend.input.mouse_button_down_event(button);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, _x: f32, _y: f32) 
    {
		self.egui_backend.input.mouse_button_up_event(button);
	}

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) 
    {
		self.egui_backend.input.mouse_motion_event(x, y);
	}
}


pub fn main() -> GameResult 
{
    let cb = ggez::ContextBuilder::new("PONGGERS", "ggez")
    .window_setup(ggez::conf::WindowSetup::default().title("PONGGERS").vsync(true))
    .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT + 100.0));

    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

