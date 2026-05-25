use crate::models::gcode::GCodeOptions;

use super::geometry::Entity;
use super::point::Point;

pub struct Finisher{
    security_z: f64,
}

impl Finisher {
    pub fn new(security_z: f64) -> Self {
        Self { security_z }
    }
}

impl Entity for Finisher {
    fn gcode_path(&self, _options: GCodeOptions) -> String {
        format!("G0 Z{:.3}\n", self.security_z)
    }

    fn end(&self) -> Point {
        Point { x: 0.0, y: 0.0, z: self.security_z, layer: String::new() }
    }

    fn revert(&self) -> Box<dyn Entity> {
        Box::new(Self::new(self.security_z))
    }

    fn start(&self) -> Point {
        Point { x: 0.0, y: 0.0, z: self.security_z, layer: String::new() }
    }
}