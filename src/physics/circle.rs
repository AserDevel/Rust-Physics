use std::f64::consts::PI;

use graphics::color;
use graphics::math::Matrix2d;
use graphics::Transformed;

use crate::Vector2f;
use crate::physics::shape::Renderable;
use crate::GlGraphics;
use crate::physics::shape::Shape;

use super::collision::AABB;

#[derive(Clone, Copy)]
pub struct Circle {
    pub radius: f64,
    pub center: Vector2f<f64>,
    pub rotation: f64,
    pub color: [f32; 4],
}

impl Renderable for Circle {
    fn draw(&self, transform: Matrix2d, gl: &mut GlGraphics) {
        let square = graphics::rectangle::centered_square(0.0, 0.0, self.radius);
        let transform = transform.trans_pos(self.center).rot_rad(self.rotation);

        graphics::ellipse(self.color, square, transform, gl);

        let top_point = Vector2f::new(0.0, -self.radius);
        let line = [0.0, 0.0, top_point.x, top_point.y];
        graphics::line(color::BLACK, 5.0, line, transform, gl);
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }

    fn momemnt_of_inertia(&self) -> f64 {
        PI * f64::powi(self.radius, 4) / 4.0
    }

    fn contains_point(&self, point: Vector2f<f64>) -> bool {
        (self.center - point).len() <= self.radius
    }
}

impl Circle {
    pub fn new(center: Vector2f<f64>, radius: f64, color: [f32; 4]) -> Self {
        Self { 
            radius: radius, 
            center: center, 
            rotation: 0.0,
            color: color, 
        }
    }

    // Returns closest surface point and surface normal
    pub fn find_closest_surface_point(&self, point: Vector2f<f64>) -> (Vector2f<f64>, Vector2f<f64>) {
        let cp = self.center + (point - self.center).normalize() * self.radius;
        let normal = (cp - self.center).normalize();
        (cp, normal)
    }

    pub fn get_aabb(&self) -> AABB {
        let dims = Vector2f::new(self.radius, self.radius);
        AABB { top_left: self.center - dims, bottom_right: self.center + dims}
    }
}