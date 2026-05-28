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