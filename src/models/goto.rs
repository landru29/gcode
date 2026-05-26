use super::{
    geometry::{Entity, Filtered},
    point::Point,
    gcode::GCodePathOptions,
};

#[derive(Clone, PartialEq)]
pub struct Goto(Point);

impl Goto {
    pub fn new(point: Point) -> Self {
        Self(point)
    }

    pub fn square_distance(&self, other: &Self) -> f64 {
        self.0.square_distance(&other.0)
    }
}

impl Entity for Goto {
    fn start(&self) -> Point {
        self.0.clone()
    }

    fn end(&self) -> Point {
        self.0.clone()
    }

    fn revert(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }

    fn gcode_path(&self, gcode_options: GCodePathOptions) -> String {
        let starter = if gcode_options.goto_start {
            format!("G0 Z{:.3}\n", gcode_options.security_z)
        } else {
            "".to_string()
        };

        let mut options = gcode_options.clone();
        options.feed = 0.0;
        options.arc_center = None;

        format!(
            "{}G0 {}\n",
            starter,
            options.parameters_string(&self.end())
        )
    }
}
