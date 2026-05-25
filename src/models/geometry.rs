use super::gcode::GCodeOptions;
use super::point::Point;

pub trait Entity {
    fn start(&self) -> Point;
    fn end(&self) -> Point;
    fn revert(&self) -> Box<dyn Entity>;
    fn gcode_path(&self, options: GCodeOptions) -> String;
}


pub trait Layered {
    fn layer(&self) -> String;
}