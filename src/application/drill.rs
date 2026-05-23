use crate::{
    application::sort::nearest_entity, 
    models::{
        entity::Entity,
        entity_set::EntitySet,
        point::Point,
    }
};


pub fn drill_sequence(entities: EntitySet) -> EntitySet {
    let mut output = EntitySet::default();

    let  list: Vec<Entity> = entities.clone().into();

    let mut point_list: Vec<Point> = list.into_iter().map(|element| element.end()).collect();

    let mut reference = Point::new(0.0, 0.0, 0.0, "".to_string());

    while point_list.len()>0 {
        point_list.sort_by(|a, b| nearest_entity(&reference, Entity::Point(a.clone()), Entity::Point(b.clone())));

        let target = point_list.pop().unwrap();

        reference = target.clone();

        output.push(Entity::Point(target));
    }

    output
}



