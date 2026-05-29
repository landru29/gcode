use crate::{
    application::sort::nearest_entity, 
    models::{
        entity::Entity,
        entity_set::EntitySet,
        gcode::{GCodePathOptions, step_array},
        point::Point,
    }
};

pub fn drill_gcode(entities: EntitySet, security_z: f64, feed: f64, deep: f64, step: f64) -> String {
    let entity_set = build_drill_sequence(entities, security_z);

    let deeps = step_array(deep, step);

    let options = GCodePathOptions::default()
            .with_security_z(security_z)
            .with_feed(feed)
            .with_goto_start()
            .with_x()
            .with_y()
            .with_z();

    let tooling: String = deeps
        .iter()
        .enumerate()
        .map(|(index, this_deep)| {
            format!(
                "; **** #{:03} / {:03} Deep: {:.3} >>>>\n{}\n; <<<< #{:03} / {:03} ****\n\n", 
                index+1, 
                deeps.len(),
                this_deep,
                entity_set.gcode_path(options.clone().with_override_z(-this_deep)),
                index+1, 
                deeps.len(),
            )
        }).collect();

    format!("{}\n{}{}",
        Entity::Starter.gcode_path(options.clone()),
        tooling,
        Entity::Finisher.gcode_path(options.clone()),
    )
}

fn build_drill_sequence(entities: EntitySet, security_z: f64) -> EntitySet {
    let mut output = EntitySet::default();

    let  list: Vec<Entity> = entities.clone().into();

    let mut point_list: Vec<Point> = list.into_iter().map(|element| element.end()).collect();

    let mut reference = Point::new(0.0, 0.0, 0.0, "".to_string());

    while point_list.len()>0 {
        point_list.sort_by(|a, b| nearest_entity(&reference, Entity::Point(a.clone()), Entity::Point(b.clone())));

        let target = point_list.pop().unwrap();

        let mut starting_point =  target.clone();
        starting_point.z = security_z;

        reference = target.clone();

        output.push(Entity::Point(target));
    }

    output
}



