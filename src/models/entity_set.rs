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
                format!("; ____________ {:03} / {:03} ____________\n{}", index+1, total, e.gcode_path(gcode_options.clone()))
            })
            .collect();

        format!("{}", output.join("\n"))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn generate_code(&self, options: super::gcode::GCodePathOptions, deep: f64, step: f64) -> String {
        let deeps = step_array(deep, step);

        let tooling: String = deeps
            .iter()
            .enumerate()
            .map(|(index, this_deep)| {
                format!(
                    ";    *****************************\n;    ** {:03} / {:03} Deep: {:7.3} **\n;    *****************************\n{}\n\n", 
                    index+1, 
                    deeps.len(),
                    this_deep,
                    self.gcode_path(options.clone().with_override_z(-this_deep)),
                )
            }).collect();

        format!("{}\n{}{}",
            Entity::Starter.gcode_path(options.clone()),
            tooling,
            Entity::Finisher.gcode_path(options.clone()),
        )
    }
}


// Utility function to generate an array of steps from 0 to deep with a given step size
fn step_array(deep: f64, step: f64) -> Vec<f64> {
    let mut steps = vec![];
    let mut current = step;

    while current < deep {
        steps.push(current);
        current += step;
    }

    steps.push(deep);

    steps
}