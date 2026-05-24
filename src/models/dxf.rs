use super::point::Point as LocalPoint;

use dxf::{Point, entities::*};


impl From<&Line> for super::line::Line {
    fn from(line: &Line) -> Self {
        Self {
            start: LocalPoint::new(line.p1.x, line.p1.y, line.p1.z),
            end: LocalPoint::new(line.p2.x, line.p2.y, line.p2.z),
        }
    }
}

impl From<&Arc> for super::arc::Arc {
    fn from(arc: &Arc) -> Self {
        Self {
            center: LocalPoint::new(arc.center.x, arc.center.y, arc.center.z),
            radius: arc.radius,
            start_angle: arc.start_angle,
            end_angle: arc.end_angle,
            clockwise: arc.start_angle > arc.end_angle,
        }
    }
}

impl From<&Point> for LocalPoint {
    fn from(point: &Point) -> Self {
        Self::new(point.x, point.y, point.z)
    }
}