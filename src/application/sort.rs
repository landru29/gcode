use std::cmp::Ordering;


use crate::models::{
    entity::Entity, point::Point
};

pub fn nearest_entity(reference: &Point, a: Entity, b: Entity) -> Ordering {
    if reference.square_distance(&a) < reference.square_distance(&b) {
        Ordering::Greater
    } else if reference.square_distance(&a) > reference.square_distance(&b) {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}
