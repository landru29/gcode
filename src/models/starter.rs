use super::geometry::Entity;
use super::point::Point;

pub struct Starter{
    security_z: f64,
}

impl Starter {
    pub fn new(security_z: f64) -> Self {
        Self { security_z }
    }
}

impl Entity for Starter {
    fn to_gcode(&self, _speed: f64, _goto_start: bool) -> String {
        format!("G90\nG21\nG0 Z{:.3}\n", self.security_z)
    }

    fn end(&self) -> Point {
        Point { x: 0.0, y: 0.0, z: self.security_z }
    }

    fn revert(&self) -> Box<dyn Entity> {
        Box::new(Self::new(self.security_z))
    }

    fn start(&self) -> Point {
        Point { x: 0.0, y: 0.0, z: self.security_z }
    }
}