use super::point::Point as LocalPoint;
use dxf::{Point, entities::*};


pub struct Layered<T> {
    pub layer: String,
    pub entity: T,
}

impl From<Layered<&Line>> for super::line::Line {
    fn from(line: Layered<&Line>) -> Self {
        Self::new(
            LocalPoint::new(line.entity.p1.x, line.entity.p1.y, line.entity.p1.z, line.layer.clone()),
            LocalPoint::new(line.entity.p2.x, line.entity.p2.y, line.entity.p2.z, line.layer.clone()),
            line.layer.clone()
        )
    }
}

impl From<Layered<&Arc>> for super::arc::Arc {
    fn from(arc: Layered<&Arc>) -> Self {
        Self::new(
             LocalPoint::new(arc.entity.center.x, arc.entity.center.y, arc.entity.center.z, arc.layer.clone()),
             arc.entity.radius,
             arc.entity.start_angle,
             arc.entity.end_angle,
             arc.entity.start_angle > arc.entity.end_angle,
             arc.layer.clone()
        )
    }
}

impl From<Layered<&Point>> for LocalPoint {
    fn from(point: Layered<&Point>) -> Self {
        Self::new(point.entity.x, point.entity.y, point.entity.z, point.layer.clone())
    }
}