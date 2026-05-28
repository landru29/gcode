use crate::{
    application::sort::nearest_dyn_entity, 
    models::{
        entity::{Entity, EntitySet}, 
        finisher::Finisher, 
        gcode::GCodePathOptions, 
        multiline::{InsertionMode, Multiline}, 
        point::Point, 
        starter::Starter,
    }
};


pub fn path_gcode(entities: impl IntoIterator<Item = Box<dyn Entity>>, security_z: f64, feed: f64, deep: f64) -> String {
    let mut output = EntitySet::default();

    output.push(Box::new(Starter::default()));

    for multiline in  build_path(entities) {
        output.push(Box::new(multiline));
    }

    output.push(Box::new(Finisher::default()));


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


fn build_path(entities: impl IntoIterator<Item = Box<dyn Entity>>) -> Vec<Multiline> {
    let mut output: Vec<Multiline> = vec![];

    let mut entities: Vec<Box<dyn Entity>> = entities.into_iter().map(|element| element).collect();

    let mut reference = Point::new(0.0, 0.0, 0.0, "".to_string());
    
    while entities.len()>0 {
        match extract_multiline(reference.clone(), entities) {
            Some(multiline) => {
                reference = multiline.end();
                output.push(multiline);
            },
            None => {
                return output
            }
        };
    }

    output
}


fn extract_multiline(start: Point, mut entities: Vec<Box<dyn Entity>>) -> Option<Multiline> {
    if entities.len() == 0 {
        None
    } else {
        let mut output = Multiline::default();

        let mut reference = start.clone();

        while entities.len()>0 {
            entities.sort_by(|a, b| nearest_dyn_entity(&reference, a, b));

            let entity = entities.drain(0..1).next().unwrap();
            reference = entity.end();

            match output.can_insert(&entity) {
                InsertionMode::None => return Some(output),
                _ => {
                    output.add_entity(entity)
                }
            };
        }

        Some(output)
    }
}
