use crate::{
    application::sort::nearest_entity, 
    models::{
        entity::{Entity, EntitySet},
        finisher::Finisher,
        gcode::GCodePathOptions,
        goto::Goto,
        line::Line,
        point::Point,
        starter::Starter,
    }
};

fn build_drill_sequence(entities: impl IntoIterator<Item = Box<dyn Entity>>, security_z: f64, deep: f64) -> EntitySet {
    let mut output = EntitySet::default();

    let mut point_list: Vec<Point> = entities.into_iter().map(|element| element.end().with_z(-deep)).collect();

    output.push(Box::new(Starter::default()));

    let mut reference = Point::new(0.0, 0.0, 0.0, "".to_string());

    while point_list.len()>0 {
        point_list.sort_by(|a, b| nearest_entity(&reference, a, b));

        let target = point_list.pop().unwrap();

        let mut starting_point =  target.clone();
        starting_point.z = security_z;

        reference = target.clone();

        output.push(Box::new(Goto::new(starting_point.clone())));
        output.push(Box::new(Line::new(starting_point, target, "".to_string())));
    }

    output.push(Box::new(Finisher::default()));
    
    output
}

pub fn drill_gcode(entities: impl IntoIterator<Item = Box<dyn Entity>>, security_z: f64, feed: f64, deep: f64) -> String {
    let entity_set = build_drill_sequence(entities, security_z, deep);

    entity_set.gcode_path(
        GCodePathOptions::default()
            .with_security_z(security_z)
            .with_feed(feed)
            .with_goto_start()
            .with_x()
            .with_y()
            .with_z()
    )
}

