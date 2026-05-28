use thiserror::Error;

use crate::models::entity::Entity;

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

#[derive(Default, Clone)]
pub struct Multiline(Vec<Entity>);

impl From<Vec<Entity>> for Multiline {
    fn from(value: Vec<Entity>) -> Self {
        Self(value)
    }
}

impl Into<Vec<Entity>> for Multiline {
    fn into(self) -> Vec<Entity> {
        self.0
    }
}


impl Multiline {
    pub fn can_insert(&self, entity: Entity) -> InsertionMode {
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

    fn can_insert_at_start(&self, entity: Entity)-> InsertionMode {
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

    pub fn add_entity(&mut self, entity: Entity) -> Result<(), MultilineError> {
        match self.can_insert(entity.clone()) {
            InsertionMode::InsertAtEnd => {
                self.0.push(entity.clone());
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

    pub fn start(&self) -> crate::models::point::Point {
        self.0.first().unwrap().start()
    }

    pub fn end(&self) -> crate::models::point::Point {
        self.0.last().unwrap().end()
    }
}