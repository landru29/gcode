use super::{
    entity::Entity,
};


#[derive(Default, Clone)]
pub struct EntitySet(Vec<Entity>);

impl From<Vec<Entity>> for EntitySet {
    fn from(value: Vec<Entity>) -> Self {
        Self(value)
    }
}

impl Into<Vec<Entity>> for EntitySet {
    fn into(self) -> Vec<Entity> {
        self.0
    }
}

impl EntitySet {
    pub fn push(&mut self, entity: Entity) {
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
            .map(|(index, e)| {
                format!("; #{:03} / {:03}\n{}", index+1, total, e.gcode_path(gcode_options.clone()))
            })
            .collect();

        format!("{}", output.join("\n"))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
