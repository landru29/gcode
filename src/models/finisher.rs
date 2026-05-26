use crate::models::gcode::GCodePathOptions;

use super::geometry::Entity;
use super::point::Point;

pub struct Finisher{}

impl Finisher {
    pub fn new() -> Self {
        Self {}
    }
}

impl Entity for Finisher {
    fn gcode_path(&self, options: GCodePathOptions) -> String {
        format!("; ending\nG0 Z{:.3}\n", options.security_z)
    }

    fn end(&self) -> Point {
        Point { x: 0.0, y: 0.0, z: 0.0, layer: String::new() }
    }

    fn revert(&self) -> Box<dyn Entity> {
        Box::new(Self::new())
    }

    fn start(&self) -> Point {
        Point { x: 0.0, y: 0.0, z: 0.0, layer: String::new() }
    }
}