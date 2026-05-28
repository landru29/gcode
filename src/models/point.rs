use super::{
    entity::Entity,
};

use super::{
    filter::Filtered,
};

#[derive(Clone, PartialEq, Default)]
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

    pub fn square_distance(&self, other: &Entity) -> f64 {
        (self.x - other.start().x) * (self.x - other.start().x) +
        (self.y - other.start().y) * (self.y - other.start().y) +
        (self.z - other.start().z) * (self.z - other.start().z).min(
            (self.x - other.end().x) * (self.x - other.end().x) +
            (self.y - other.end().y) * (self.y - other.end().y) +
            (self.z - other.end().z) * (self.z - other.end().z)
        )
    }

    pub fn with_z(&self, z: f64) -> Self {
        let mut output = self.clone();
        output.z = z;

        output
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