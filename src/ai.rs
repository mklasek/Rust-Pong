use ggez::{Context, GameResult};

use crate::ball::Ball;
use crate::paddle::Paddle;

pub struct AIPlayer
{
    pub paddle: Paddle,
    pub max_speed: f32,
    pub score: usize,
}

//new, draw, update
impl AIPlayer
{
    pub fn new(paddle: Paddle, max: f32) -> Self
    {
        let ai = Self 
        {
            paddle: paddle,
            max_speed: max,
            score: 0,
        };

        return ai;
    }
    pub fn update(&mut self, ctx: &mut Context, dt: std::time::Duration, ball: &Ball) -> GameResult
    {
        let ball_pos = ball.get_position();
        let paddle_y = self.paddle.get_rect().y;

        if ball_pos.y - paddle_y > 5.0
        {
            self.paddle.move_down(self.max_speed);
        }
        else if ball_pos.y - paddle_y < -5.0
        {
            self.paddle.move_up(self.max_speed);
        }
        else
        {
            self.paddle.stop();
        }
        self.paddle.update(ctx, dt)?;
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult 
    {
        self.paddle.draw(ctx)?;
        Ok(())
    }
}