use crate::{
    application::sort::nearest_entity, 
    models::{
        entity::Entity,
        entity_set::EntitySet,
        gcode::GCodePathOptions,
        line::Line,
        point::Point,
    }
};

fn build_drill_sequence(entities: EntitySet, security_z: f64, deep: f64) -> EntitySet {
    let mut output = EntitySet::default();

    let  list: Vec<Entity> = entities.clone().into();

    let mut point_list: Vec<Point> = list.into_iter().map(|element| element.end().with_z(-deep)).collect();

    output.push(Entity::Starter);

    let mut reference = Point::new(0.0, 0.0, 0.0, "".to_string());

    while point_list.len()>0 {
        point_list.sort_by(|a, b| nearest_entity(&reference, Entity::Point(a.clone()), Entity::Point(b.clone())));

        let target = point_list.pop().unwrap();

        let mut starting_point =  target.clone();
        starting_point.z = security_z;

        reference = target.clone();

        output.push(Entity::Goto(starting_point.clone()));
        output.push(Entity::Line(Line::new(starting_point, target, "".to_string())));
    }

    output.push(Entity::Finisher);
    
    output
}

pub fn drill_gcode(entities: EntitySet, security_z: f64, feed: f64, deep: f64) -> String {
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

