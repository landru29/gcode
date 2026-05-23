use crate::{
    application::sort::nearest_entity,
    models::{
        entity::Entity,
        entity_set::EntitySet,
        multiline::{InsertionMode, Multiline},
        point::Point,
    }
};


pub fn path_sequence(entities: EntitySet) -> EntitySet {
    let mut output: EntitySet = EntitySet::default();

    for multiline in  build_path(&entities) {
        output.push(Entity::Multiline(multiline));
    }

    output
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
