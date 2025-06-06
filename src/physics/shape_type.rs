use graphics::math::Matrix2d;

use crate::Vector2f;
use crate::physics::shape::Renderable;
use crate::physics::circle::Circle;
use crate::physics::polygon::Polygon;
use crate::GlGraphics;
use crate::physics::shape::Shape;

use super::collision::AABB;

#[derive(Clone)]
pub enum ShapeType {
    Circle(Circle),
    Polygon(Polygon),
}

impl Renderable for ShapeType {
    fn draw(&self, transform: Matrix2d, gl: &mut GlGraphics, color: [f32; 4]) {
        match self {
            ShapeType::Circle(circle) => circle.draw(transform, gl, color),
            ShapeType::Polygon(poly) => poly.draw(transform, gl, color),
        }
    }
}

impl Shape for ShapeType {
    fn area(&self) -> f64 {
        match self {
            ShapeType::Circle(c) => c.area(),
            ShapeType::Polygon(p) => p.area(),
        }
    }   

    fn momemnt_of_inertia(&self) -> f64 {
        match self {
            ShapeType::Circle(c) => c.momemnt_of_inertia(),
            ShapeType::Polygon(p) => p.momemnt_of_inertia(),
        }
    }

    fn get_aabb(&self) -> AABB {
        match self {
            ShapeType::Circle(c) => c.get_aabb(),
            ShapeType::Polygon(p) => p.get_aabb(),
        }
    }

    fn contains_point(&self, point: Vector2f<f64>) -> bool {
        match self {
            ShapeType::Circle(c) => c.contains_point(point),
            ShapeType::Polygon(p) => p.contains_point(point)
        }
    }

    fn find_closest_surface_point(&self, point: Vector2f<f64>) -> (Vector2f<f64>, Vector2f<f64>) {
        match self {
            ShapeType::Circle(c) => c.find_closest_surface_point(point),
            ShapeType::Polygon(p) => p.find_closest_surface_point(point),
        }
    }
}

impl ShapeType {
    pub fn get_center(&self) -> Vector2f<f64> {
        match self {
            ShapeType::Circle(c) => c.center,
            ShapeType::Polygon(p) => p.center,
        }
    }
    
    pub fn set_center(&mut self, position: Vector2f<f64>) {
        match self {
            ShapeType::Circle(c) => c.center = position,
            ShapeType::Polygon(p) => p.center = position,
        }
    }

    pub fn translate(&mut self, translation: Vector2f<f64>) {
        match self {
            ShapeType::Circle(c) => c.center += translation,
            ShapeType::Polygon(p) => p.center += translation,
        }
    }

    pub fn get_rotation(&self) -> f64 {
        match self {
            ShapeType::Circle(c) => c.rotation,
            ShapeType::Polygon(p) => p.rotation,
        }
    } 

    #[allow(dead_code)]
    pub fn set_rotation(&mut self, rotation: f64) {
        match self {
            ShapeType::Circle(c) => c.rotation = rotation,
            ShapeType::Polygon(p) => p.rotation = rotation,
        }
    }

    pub fn rotate(&mut self, radians: f64) {
        match self {
            ShapeType::Circle(c) => c.rotation += radians,
            ShapeType::Polygon(p) => p.rotation += radians,
        }
    }

    // Returns a clone of the given shape scaled by the ratio
    pub fn scale(&self, ratio: f64) -> Self {
        match self {
            ShapeType::Circle(c) => ShapeType::Circle(Circle::new(c.center, c.radius * ratio, c.rotation)),
            ShapeType::Polygon(p) => {
                let verts = p.local_vertices.iter().map(|&v| v * ratio).collect();
                ShapeType::Polygon(Polygon::new(verts, p.center, p.rotation))
            } 
        }
    }
}