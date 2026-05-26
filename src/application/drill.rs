use std::cmp::Ordering;

use crate::models::{
    finisher::Finisher, gcode::GCodePathOptions, geometry::{Entity, EntitySet}, goto::Goto, line::Line, point::Point, starter::Starter
};

fn build_drill_sequence(entities: Vec<Box<dyn Entity>>, security_z: f64, deep: f64) -> EntitySet {
    let mut output = EntitySet::new();

    let mut point_list: Vec<Point> = entities.iter().map(|element| element.end().with_z(-deep)).collect();

    output.push(Box::new(Starter::new()));

    let mut reference = Point::new(0.0, 0.0, 0.0, "".to_string());

    while point_list.len()>0 {
        point_list.sort_by(|a, b| nearest_point(&reference, &a, &b));

        let target = point_list.pop().unwrap();

        let mut starting_point =  target.clone();
        starting_point.z = security_z;

        reference = target.clone();

        output.push(Box::new(Goto::new(starting_point.clone())));
        output.push(Box::new(Line::new(starting_point, target, "".to_string())));
    }

    output.push(Box::new(Finisher::new()));
    
    output
}

pub fn drill_gcode(entities: Vec<Box<dyn Entity>>, security_z: f64, feed: f64, deep: f64) -> String {
    let entity_set = build_drill_sequence(entities, security_z, deep);

    entity_set.gcode_path(GCodePathOptions::new(&vec![
        GCodePathOptions::with_security_z(&security_z),
        GCodePathOptions::with_feed(&feed),
        GCodePathOptions::with_goto_start(),
        GCodePathOptions::with_x(),
        GCodePathOptions::with_y(),
        GCodePathOptions::with_z(),
    ]))
}

fn nearest_point(reference: &Point, a: &Point, b: &Point) -> Ordering {
    if reference.square_distance(a) > reference.square_distance(b) {
        Ordering::Greater
    } else if reference.square_distance(a) < reference.square_distance(b) {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}