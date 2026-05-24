use super::{
    geometry::Entity,
    point::Point,
};

#[derive(Clone)]
pub struct Arc {
    pub center: Point,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
    pub clockwise: bool,
}

impl Arc {
    pub fn new(center: Point, radius: f64, start_angle: f64, end_angle: f64, clockwise: bool) -> Self {
        Self {
            center,
            radius,
            start_angle,
            end_angle,
            clockwise,
        }
    }
}


impl  Entity for Arc {
    fn start(&self) -> Point {
        Point {
            x: self.center.x + self.radius * self.start_angle.cos(),
            y: self.center.y + self.radius * self.start_angle.sin(),
            z: self.center.z,
        }
    }

    fn end(&self) -> Point {
        Point {
            x: self.center.x + self.radius * self.end_angle.cos(),
            y: self.center.y + self.radius * self.end_angle.sin(),
            z: self.center.z,
        }
    }

    fn revert(&self) -> Box<dyn Entity> {
        Box::new(Self {
            center: self.center.clone(),
            radius: self.radius,
            start_angle: self.end_angle,
            end_angle: self.start_angle,
            clockwise: !self.clockwise,
        })
    }

    fn to_gcode(&self, speed: f64, goto_start: bool) -> String {
        let starter = if goto_start {
            format!(
                "G0 X{:.3} Y{:.3} Z{:.3}\n",
                self.start().x, self.start().y, self.start().z
            )
        } else {
            "".to_string()
        };

        if self.start_angle == self.end_angle {
            let arc1 = Self {
                center: self.center.clone(),
                radius: self.radius,
                start_angle: self.start_angle,
                end_angle: self.start_angle + std::f64::consts::PI,
                clockwise: self.clockwise,
            };
            // Full circle, use G2/G3 with the same start and end points
            let arc2 = Self {
                center: self.center.clone(),
                radius: self.radius,
                start_angle: self.start_angle + std::f64::consts::PI,
                end_angle: self.start_angle + 2.0 * std::f64::consts::PI,
                clockwise: self.clockwise,
            };
            format!(
                "{}{}",
                arc1.to_gcode(speed, false),
                arc2.to_gcode(speed, false)
            )
        } else {

        let i = self.center.x - self.start().x;
        let j = self.center.y - self.start().y;
        format!(
            "{} {} X{:.3} Y{:.3} I{:.3} J{:.3} F{:.1}\n",
            starter,
            if self.clockwise { "G2" } else { "G3" },
            self.end().x, self.end().y, i, j, speed
        )
    }
    }
}