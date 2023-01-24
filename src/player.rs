use ggez::{Context, GameResult};

use crate::paddle::Paddle;

pub struct Player
{
    pub paddle: Paddle,
    pub max_speed: f32,
    pub score: usize,
}

//new, draw, update
impl Player
{
    pub fn new(paddle: Paddle, max: f32) -> Self
    {
        let player = Self 
        {
            paddle: paddle,
            max_speed: max,
            score: 0,
        };

        return player;
    }
    pub fn update(&mut self, ctx: &mut Context, dt: std::time::Duration) -> GameResult
    {
        self.paddle.update(ctx, dt)?;
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult 
    {
        self.paddle.draw(ctx)?;
        Ok(())
    }
}