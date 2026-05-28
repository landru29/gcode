use crate::models::multiline::Multiline;

use super::gcode::GCodePathOptions;
use super::point::Point;

pub trait Entity {
    fn start(&self) -> Point;
    fn end(&self) -> Point;
    fn revert(&self) -> Box<dyn Entity>;
    fn gcode_path(&self, options: GCodePathOptions) -> String;
}

#[derive(Default)]
pub struct EntitySet(Vec<Box<dyn Entity>>);

impl EntitySet {
    pub fn push(&mut self, entity: Box<dyn Entity>) {
        self.0.push(entity);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn gcode_path(&self, gcode_options: super::gcode::GCodePathOptions) -> String {
        let total = self.len();
        let output: Vec<String> = self.0
            .iter()
            .enumerate()
            .map(|(index, e)| format!("; #{:03} / {:03}\n{}", index, total, e.gcode_path(gcode_options.clone())))
            .collect();

        format!("{}", output.join("\n"))
    }
}



