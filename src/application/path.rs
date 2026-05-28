use crate::{
    application::sort::nearest_entity,
    models::{
        entity::Entity,
        entity_set::EntitySet,
        gcode::GCodePathOptions,
        multiline::{InsertionMode, Multiline},
        point::Point,
    }
};


pub fn path_gcode(entities: &EntitySet, security_z: f64, feed: f64, deep: f64) -> String {
    let mut output = EntitySet::default();

    output.push(Entity::Starter);

    for multiline in  build_path(entities) {
        output.push(Entity::Multiline(multiline));
    }

    output.push(Entity::Finisher);


    output.gcode_path(
        GCodePathOptions::default()
            .with_security_z(security_z)
            .with_feed(feed)
            .with_goto_start()
            .with_x()
            .with_y()
            .with_z()
    )
}

fn build_path(entities: &EntitySet) -> Vec<Multiline> {
    let mut output: Vec<Multiline> = vec![];
    let mut reference = Point::new(0.0, 0.0, 0.0, "".to_string());
    let mut entities = entities.clone();

    while !entities.is_empty() {
        match extract_multiline(reference.clone(), entities.clone()) {
            Some((multiline, remaining)) => {
                reference = multiline.end();
                output.push(multiline);
                entities = remaining;
            }
            _ => {}
        };
    }

    output
}


fn extract_multiline(start: Point, working_list: EntitySet) -> Option<(Multiline, EntitySet)> {
     if working_list.len() == 0 {
        None
    } else {
        let mut output = Multiline::default();
        let mut reference = start.clone();
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
                            reference = entity.end();
                            match output.add_entity(entity) {
                                Err(_) => return Some((output, EntitySet::from(list))),
                                _ => {},
                            }
                         },
                    };
                },
            };
        };

        None
    }
}
