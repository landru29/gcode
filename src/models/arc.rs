use std::fmt;

use super::{
    filter::Filtered,
    point::Point,
    gcode::GCodePathOptions,
};

#[derive(Clone, Default)]
pub struct Arc {
    pub center: Point,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
    pub clockwise: bool,
    pub layer: String,
}

impl Arc {
    pub fn new(center: Point, radius: f64, start_angle_degree: f64, end_angle_degree: f64, clockwise: bool, layer: String) -> Self {
        Self {
            center,
            radius,
            start_angle: start_angle_degree * std::f64::consts::PI/180.0,
            end_angle: end_angle_degree * std::f64::consts::PI/180.0,
            clockwise,
            layer,
        }
    }

    fn start(&self) -> Point {
        Point {
            x: self.center.x + self.radius * self.start_angle.cos(),
            y: self.center.y + self.radius * self.start_angle.sin(),
            z: self.center.z,
            layer: self.layer.clone(),
        }
    }

    fn end(&self) -> Point {
        Point {
            x: self.center.x + self.radius * self.end_angle.cos(),
            y: self.center.y + self.radius * self.end_angle.sin(),
            z: self.center.z,
            layer: self.layer.clone(),
        }
    }

    pub fn gcode_path(&self, gcode_options: GCodePathOptions) -> String {
        let starter = gcode_options.transition_to(&self.start());

        if self.start_angle == self.end_angle {
            let arc1 = Self {
                center: self.center.clone(),
                radius: self.radius,
                start_angle: self.start_angle,
                end_angle: self.start_angle + std::f64::consts::PI,
                clockwise: self.clockwise,
                layer: self.layer.clone(),
            };
            // Full circle, use G2/G3 with the same start and end points
            let arc2 = Self {
                center: self.center.clone(),
                radius: self.radius,
                start_angle: self.start_angle + std::f64::consts::PI,
                end_angle: self.start_angle + 2.0 * std::f64::consts::PI,
                clockwise: self.clockwise,
                layer: self.layer.clone(),
            };
            format!(
                "; {}\n{}{}",
                self,
                arc1.gcode_path(gcode_options.clone()),
                arc2.gcode_path(gcode_options.clone())
            )
        } else {
            let i = self.center.x - self.start().x;
            let j = self.center.y - self.start().y;
            format!(
                "; {}\n{}{} {} I{:.3} J{:.3}\n",
                self,
                starter,
                if self.clockwise { "G2" } else { "G3" },
                gcode_options.parameters_string(&self.end()), 
                i, 
                j,
            )
        }
    }
}

impl Filtered for Arc {
    fn layer(&self) -> String {
        self.layer.clone()
    }

        fn entity_type(&self) -> String {
        "arc".to_string()
    }
}

impl fmt::Display for Arc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start() == self.end() {
            write!(
                f, "Circle [{:.3}, {:.3}] @ {:.3}",
                self.center.x, self.center.y , self.radius,
            )
        } else {
            write!(f, "Arc [{:.3}, {:.3}] -> [{:.3}, {:.3}]", 
                self.start().x, self.start().y,
                self.end().x, self.end().y,
            )
        }
    }
}

#[test]
fn test_start_end() {
    let arc = Arc::new(
        Point::new(3.0, 2.0, 0.0, String::from("")),
        2.0,
        -90.0,
        -180.0,
        false,
        String::from("")
    );

    assert_eq!(arc.start(), Point::new(3.0, 0.0, 0.0, String::from("")), "testing start point");
    assert_eq!(arc.end(), Point::new(1.0, 2.0, 0.0, String::from("")), "testing end point");
}