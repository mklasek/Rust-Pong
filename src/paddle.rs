use glam::Vec2;
use ggez::{Context, GameResult};
use ggez::graphics;

use crate::WINDOW_HEIGHT;

pub struct Paddle
{
    position: Vec2,
    width: f32,
    height: f32,
    velocity: f32,
    low_limit: f32,
    high_limit: f32,
    mesh: graphics::Mesh,
}

//new, draw, update
impl Paddle
{
    pub fn new(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, edge: f32) -> Self
    {
        let origin_x = 0.0 - w / 2.0;
        let origin_y = 0.0 - h/ 2.0;
        let rect = graphics::Rect::new(origin_x, origin_y, w, h);
        let mesh = graphics::Mesh::new_rectangle(ctx, 
                                                 graphics::DrawMode::fill(), 
                                                 rect, 
                                                 graphics::Color::YELLOW);
        let paddle = Paddle 
        {
            position: Vec2::new(x, y),
            width: w,
            height: h,
            velocity: 0.0,
            low_limit: 0.0 + h / 2.0,
            high_limit: edge - h / 2.0,
            mesh: mesh.unwrap(),
        };
        return paddle;
    }

    pub fn update(&mut self, _ctx: &mut Context, dt: std::time::Duration) -> GameResult
    {
        self.position.y += self.velocity  * dt.as_secs_f32();
        if self.position.y >= self.high_limit
        {
            self.position.y = self.high_limit;
            self.velocity = 0.0;
        }
        if self.position.y <= self.low_limit
        {
            self.position.y = self.low_limit;
            self.velocity = 0.0;
        }
        return Ok(());
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult
    {
        graphics::draw(ctx, &self.mesh, (self.position, ))?;
        return Ok(());
    }
}


//getters
impl Paddle
{
    pub fn get_rect(&self) -> graphics::Rect
    {
        return graphics::Rect
        {
            x: self.position.x,
            y: self.position.y,
            w: self.width,
            h: self.height,
        };
    }

    pub fn get_velocity(&self) -> f32
    {
        return self.velocity;
    }
}

//actions
impl Paddle
{
    pub fn move_up(&mut self, speed: f32)
    {
        self.velocity = -1.0 * speed;
    }

    pub fn stop(&mut self)
    {
        self.velocity = 0.0;
    }

    pub fn move_down(&mut self, speed: f32)
    {
        self.velocity = 1.0 * speed;
    }

    pub fn reset(&mut self)
    {
        self.position.y = WINDOW_HEIGHT / 2.0;
    }
}
    

