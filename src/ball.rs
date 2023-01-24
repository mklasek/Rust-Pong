use glam::Vec2;
use ggez::{Context, GameResult};
use ggez::graphics;
use rand::random;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Ball
{
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    mesh: graphics::Mesh,
}

//new, draw, update
impl Ball
{
    pub fn new(ctx: &mut Context, x: f32, y: f32, r: f32) -> Self
    {
        let mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            r,
            0.1,
            graphics::Color::GREEN,
        );

        let ball = Ball
        {
            position: Vec2::new(x, y),
            velocity: Vec2::new(0.0, 0.0),
            radius: r,
            mesh: mesh.unwrap(),
        };
        return ball;
    }

    pub fn update(&mut self, ctx: &mut Context, dt: std::time::Duration) -> GameResult
    {
        self.position += self.velocity * dt.as_secs_f32();
        return Ok(());
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult
    {
        graphics::draw(ctx, &self.mesh, (self.position,))?;
        return Ok(());
    }
}

//getters
impl Ball
{
    pub fn get_position(&self) -> Vec2
    {
        return self.position;
    }

    pub fn get_radius(&self) -> f32
    {
        return self.radius;
    }

    pub fn get_direction(&self) -> i32
    {
        if self.velocity.x > 0.0
        {
            return 1;
        }
        else 
        {
            return -1;
        }
    }
}

//actions
impl Ball
{
    pub fn ping(&mut self)
    {
        let dx = rand::random::<f32>() * 2.0 - 1.0;
        let dy = (rand::random::<f32>() * 2.0 - 1.0) / 4.0;

        let mut v = Vec2::new(dx, dy);
        v = v.normalize();
        v = v * 500.0;

        self.velocity = v;
    }

    pub fn bounce(&mut self, normal_vec: Vec2)
    {
        let size = self.velocity.length();
        let u = self.velocity.dot(normal_vec) * normal_vec;
        let w = self.velocity - u;

        self.velocity = w - u;
        self.velocity *= (size / self.velocity.length()) * 1.1;
    }

    pub fn reset(&mut self)
    {
        self.velocity = Vec2::new(0.0, 0.0);
        self.position = Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);
    }
}




