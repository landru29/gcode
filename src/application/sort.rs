use std::cmp::Ordering;


use crate::models::{
    entity::Entity,
    point::Point,
};

pub fn nearest_entity<S: Entity, T: Entity>(reference: &Point, a: &S, b: &T) -> Ordering {
    if reference.square_distance(a) < reference.square_distance(b) {
        Ordering::Greater
    } else if reference.square_distance(a) > reference.square_distance(b) {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}
pub fn nearest_dyn_entity(reference: &Point, a: &Box<dyn Entity>, b: &Box<dyn Entity>) -> Ordering {
    let min_distance_a = reference.square_distance(&a.end()).min(reference.square_distance(&a.start()));
    let min_distance_b = reference.square_distance(&b.end()).min(reference.square_distance(&b.start()));
    if min_distance_a < min_distance_b {
        Ordering::Greater
    } else if min_distance_a > min_distance_b {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}