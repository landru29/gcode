use crate::geometry::point::Point;

pub trait Entity {
    fn start(&self) -> Point;
    fn end(&self) -> Point;
    fn revert(&self) -> Box<dyn Entity>;
    fn to_gcode(&self, speed: f64, goto_start: bool) -> String;
}