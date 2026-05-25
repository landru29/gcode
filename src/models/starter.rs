use super::{
    point::Point,
    geometry::Entity,
    gcode::GCodeOptions,
};

pub struct Starter{}

impl Starter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Entity for Starter {
    fn gcode_path(&self, _options: GCodeOptions) -> String {
        format!("G90\nG21\nG0 Z{:.3}\n", _options.security_z)
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