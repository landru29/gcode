use super::geometry::Entity;

pub struct Multiline(Vec<Box<dyn Entity>>);


impl Multiline {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) -> Option<()> {
        match self.0.last() {
            Some(last) => {
                if last.end() == entity.start() {
                    self.0.push(entity);
                    Some(())
                } else if last.end() == entity.end() {
                    self.0.push(entity.revert());
                    Some(())
                } else {
                    self.pre_add_entity(entity)
                }
            }
            None => {
                self.0.push(entity);
                Some(())
            }
        }
    }

    pub fn pre_add_entity(&mut self, entity: Box<dyn Entity>) -> Option<()> {
        match self.0.first() {
            Some(first) => {
                if first.start() == entity.end() {
                    self.0.insert(0, entity);
                    Some(())
                } else if first.start() == entity.start() {
                    self.0.insert(0, entity.revert());
                    Some(())
                } else {
                    None
                }
            }
            None => {
                self.0.insert(0, entity);
                Some(())
            }
        }
    }
}

impl Entity for Multiline {
    fn start(&self) -> crate::models::point::Point {
        self.0.first().unwrap().start()
    }

    fn end(&self) -> crate::models::point::Point {
        self.0.last().unwrap().end()
    }

    fn revert(&self) -> Box<dyn Entity> {
        let reversed = self.0.iter().rev().map(|e| e.revert()).collect::<Vec<_>>();
        Box::new(Self(reversed))
    }

    fn gcode_path(&self, gcode_options: super::gcode::GCodeOptions) -> String {
        let starter = gcode_options.transition_to(&self.start());

        let mut options = gcode_options.clone();

        options.goto_start = false;

        let output: String = self.0.iter().map(|e| e.gcode_path(options.clone())).collect();

        format!("{}{}", starter, output)
    }
}