use super::{
    geometry::{Entity, Filtered},
    gcode::GCodePathOptions,
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

    pub fn square_distance(&self, other: &Self) -> f64 {
        (self.x - other.x) * (self.x - other.x) +
        (self.y - other.y) * (self.y - other.y) +
        (self.z - other.z) * (self.z - other.z)
    }

    pub fn with_z(&self, z: f64) -> Self {
        let mut output = self.clone();
        output.z = z;

        output
    }
}

impl Entity for Point {
    fn start(&self) -> Self {
        self.clone()
    }

    fn end(&self) -> Self {
        self.clone()
    }

    fn revert(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }

    fn gcode_path(&self, gcode_options: GCodePathOptions) -> String {
        format!(
            "{}G{} {}\n",
            gcode_options.transition_to(&self.start()),
            if gcode_options.feed > 0.0 { "1" } else { "0" },
            gcode_options.parameters_string(&self.end())
        )
    }
}

impl Filtered for Point {
    fn layer(&self) -> String {
        self.layer.clone()
    }

    fn entity_type(&self) -> String {
        "point".to_string()
    }
}