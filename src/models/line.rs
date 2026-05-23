use std::fmt;

use super::{
    point::Point,
    filter::Filtered,
};

#[derive(Clone, Default)]
pub struct Line {
    pub layer: String,
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point, layer: String) -> Self {
        Self { start, end, layer }
    }
}

impl Filtered for Line {
    fn layer(&self) -> String {
        self.layer.clone()
    }

    fn entity_type(&self) -> String {
        "line".to_string()
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start.x == self.end.x && self.start.y == self.end.y {
            write!(f, "Drill {:.3} -> {:.3}", self.start.z, self.end.z)
        } else {
            write!(f, "Line [{:.3}, {:.3}] -> [{:.3}, {:.3}]", 
                self.start.x, self.start.y,
                self.end.x, self.end.y,
            )
        }
    }
}

