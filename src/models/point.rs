use super::{
    geometry::{Entity, Layered},
    gcode::GCodeOptions,
};

#[derive(Clone, PartialEq)]
pub struct Point {
    pub layer: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64, layer: String) -> Self {
        Self { x, y, z, layer }
    }
}

impl Entity for Point {
    fn start(&self) -> Point {
        self.clone()
    }

    fn end(&self) -> Point {
        self.clone()
    }

    fn revert(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }

    fn gcode_path(&self, gcode_options: GCodeOptions) -> String {
        format!(
            "{}G{} {}\n",
            gcode_options.transition_to(&self.start()),
            if gcode_options.feed > 0.0 { "1" } else { "0" },
            gcode_options.parameters_string(&self.end())
        )
    }
}

impl Layered for Point {
    fn layer(&self) -> String {
        self.layer.clone()
    }
}