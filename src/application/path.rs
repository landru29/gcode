use crate::{
    application::sort::nearest_entity,
    models::{
        entity::Entity,
        entity_set::EntitySet,
        gcode::{GCodePathOptions, step_array},
        multiline::{InsertionMode, Multiline},
        point::Point,
    }
};


pub fn path_gcode(entities: &EntitySet, security_z: f64, feed: f64, deep: f64, step: f64) -> String {
    let mut output = EntitySet::default();

    for multiline in  build_path(entities) {
        output.push(Entity::Multiline(multiline));
    }

    let options = GCodePathOptions::default()
            .with_security_z(security_z)
            .with_feed(feed)
            .with_goto_start()
            .with_x()
            .with_y()
            .with_z();

    let deeps = step_array(deep, step);

    let tooling: String = deeps
        .iter()
        .enumerate()
        .map(|(index, this_deep)| {
            format!(
                "; **** #{:03} / {:03} Deep: {:.3} >>>>\n{}\n; <<<< #{:03} / {:03} ****\n\n", 
                index+1, 
                deeps.len(),
                this_deep,
                output.gcode_path(options.clone().with_override_z(-this_deep)),
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

fn build_path(entities: &EntitySet) -> Vec<Multiline> {
    let mut output: Vec<Multiline> = vec![];
    let mut entities = entities.clone();

    while !entities.is_empty() {
        match extract_multiline( entities.clone()) {
            Some((multiline, remaining)) => {
                let mut to_insert = multiline.clone();
                let _ = to_insert.insert_at_start(Entity::Point(multiline.start()));
                output.push(to_insert);
                entities = remaining;
            }
            _ => {}
        };
    }

    output
}


fn extract_multiline(working_list: EntitySet) -> Option<(Multiline, EntitySet)> {
     if working_list.len() == 0 {
        None
    } else {
        let mut output = Multiline::default();
        let mut reference = Point::default();
        let mut list: Vec<Entity> = working_list.clone().into();

        while !working_list.is_empty() {
            list.sort_by(|a, b| nearest_entity(&reference, a.clone(), b.clone()));
            
            match list.first() {
                None => return Some((output, EntitySet::from(list))),
                Some(first) => {
                    match output.can_insert(first.clone()) {
                         InsertionMode::None => return Some((output, EntitySet::from(list))),
                         _ => {
                            let entity = list.remove(0);
                            match output.add_entity(entity) {
                                Err(_) => return Some((output, EntitySet::from(list))),
                                _ => reference = output.end(),
                            };
                         },
                    };
                },
            };
        };

        None
    }
}
