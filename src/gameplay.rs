use::glam::*;
use crate::ball::Ball;
use crate::paddle::Paddle;

pub enum Side
{
    Left,
    Right,
}

pub fn collision_detection(ball: &mut Ball, paddle1: &Paddle, paddle2: &Paddle, window_width: f32, window_height: f32)
{
    let ball_pos: Vec2 = ball.get_position();
    let ball_r: f32 = ball.get_radius();

    //bouncing off window edges
    if ball_pos.y + ball_r > window_height
    {
        ball.bounce(Vec2::new(0.0, 1.0));
        return;
    }
    else if ball_pos.y - ball_r < 0.0
    {
        ball.bounce(Vec2::new(0.0, -1.0));
        return;
    }

    //bouncing off paddles
    let rect1 = paddle1.get_rect();
    let rect2 = paddle2.get_rect();
    let dir = ball.get_direction();

    if dir == -1 && ball_pos.x < window_width * 0.25
    {
        let cond1: bool = (ball_pos.x - ball_r) <= (rect1.x + rect1.w / 2.0);
        let cond2: bool = (ball_pos.y + ball_r) <= (rect1.y + rect1.h / 2.0);
        let cond3: bool = (ball_pos.y + ball_r) >= (rect1.y - rect1.h / 2.0);

        if cond1 && cond2 && cond3
        {
            let pad = paddle1.get_velocity();
            let norm;
            if pad > 0.0
            {
                norm = Vec2::new(1.0, 0.2).normalize();
            }
            else if pad < 0.0
            {
                norm = Vec2::new(1.0, -0.2).normalize();
            }
            else
            {
                norm = Vec2::new(1.0, 0.0);
            }
            ball.bounce(norm);
        }
    }
    else if dir == 1 && ball_pos.x > window_width * 0.75
    {
        let cond1: bool = (ball_pos.x + ball_r) >= (rect2.x - rect2.w / 2.0);
        let cond2: bool = (ball_pos.y + ball_r) <= (rect2.y + rect2.h / 2.0);
        let cond3: bool = (ball_pos.y + ball_r) >= (rect2.y - rect2.h / 2.0);

        if cond1 && cond2 && cond3
        {
            let pad = paddle1.get_velocity();
            let norm;
            if pad > 0.0
            {
                norm = Vec2::new(-1.0, 0.2).normalize();
            }
            else if pad < 0.0
            {
                norm = Vec2::new(-1.0, -0.2).normalize();
            }
            else
            {
                norm = Vec2::new(-1.0, 0.0);
            }
            ball.bounce(norm);
        }
    }
}

pub fn miss_detection(ball: &Ball, window_width: f32) -> Option<Side>
{
    let ball_pos = ball.get_position();

    if ball_pos.x < 0.0
    {
        return Some(Side::Left);
    }
    else if ball_pos.x > window_width
    {
        return Some(Side::Right);
    }
    else
    {
        return None;
    }
}