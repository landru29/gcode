use super::entity::Entity;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MultilineError {
    #[error("entities are not contiguous")]
    NotContiguousEntities, 
}

pub enum InsertionMode{
    InsertAtEnd,
    RevertAndInsertAtEnd,
    InsertAtStart,
    RevertAndInsertAtStart,
    None,
}

#[derive(Default)]
pub struct Multiline(Vec<Box<dyn Entity>>);


impl Multiline {
    pub fn can_insert(&self, entity: &Box<dyn Entity>) -> InsertionMode {
        match self.0.last() {
            Some(last) => {
                if last.end() == entity.start() {
                    InsertionMode::InsertAtEnd
                } else if last.end() == entity.end() {
                    InsertionMode::RevertAndInsertAtEnd
                } else {
                    self.can_insert_at_start(entity)
                }
            }
            None => {
                InsertionMode::InsertAtEnd
            }
        }
    }

    fn can_insert_at_start(&self, entity: &Box<dyn Entity>)-> InsertionMode {
         match self.0.first() {
            Some(first) => {
                if first.start() == entity.end() {
                    InsertionMode::InsertAtStart
                } else if first.start() == entity.start() {
                    InsertionMode::InsertAtStart
                } else {
                    InsertionMode::RevertAndInsertAtStart
                }
            }
            None => {
                InsertionMode::None
            }
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) -> Result<(), MultilineError> {
        match self.can_insert_at_start(&entity) {
            InsertionMode::InsertAtEnd => {
                self.0.push(entity);
                Ok(())
            }
            InsertionMode::RevertAndInsertAtEnd => {
                self.0.push(entity.revert());
                Ok(())
            }
            InsertionMode::InsertAtStart => {
                self.0.insert(0, entity);
                Ok(())
            }
            InsertionMode::RevertAndInsertAtStart => {
                self.0.insert(0, entity.revert());
                Ok(())
            }
            InsertionMode::None => Err(MultilineError::NotContiguousEntities)
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

    fn gcode_path(&self, gcode_options: super::gcode::GCodePathOptions) -> String {
        let starter = gcode_options.transition_to(&self.start());

        let mut options = gcode_options.clone();

        options.goto_start = false;

        let output: String = self.0.iter().map(|e| e.gcode_path(options.clone())).collect();

        format!("{}{}", starter, output)
    }
}