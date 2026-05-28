use super::{
    entity::Entity, 
    point::Point,
    gcode::GCodePathOptions,
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

impl Entity for Line {
    fn start(&self) -> Point {
        self.start.clone()
    }

    fn end(&self) -> Point {
        self.end.clone()
    }

    fn revert(&self) -> Box<dyn Entity> {
        Box::new(Self {
            start: self.end.clone(),
            end: self.start.clone(),
            layer: self.layer.clone(),
        })
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

impl Filtered for Line {
    fn layer(&self) -> String {
        self.layer.clone()
    }

    fn entity_type(&self) -> String {
        "line".to_string()
    }
}