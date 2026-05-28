use super::{
    point::Point,
    entity::Entity,
    gcode::GCodePathOptions,
};

#[derive(Default)]
pub struct Starter{}


impl Entity for Starter {
    fn gcode_path(&self, options: GCodePathOptions) -> String {
        format!("; starting\nG90\nG21\nG0 Z{:.3}\n", options.security_z)
    }

    fn end(&self) -> Point {
        Point { x: 0.0, y: 0.0, z: 0.0, layer: String::new() }
    }

    fn revert(&self) -> Box<dyn Entity> {
        Box::new(Self::default())
    }

    fn start(&self) -> Point {
        Point { x: 0.0, y: 0.0, z: 0.0, layer: String::new() }
    }
}