use super::{
    point::Point,
    arc::Arc,
    line::Line,
    multiline::Multiline,
};


#[derive(Clone)]
pub enum Entity{
    Line(Line),
    Point(Point),
    Arc(Arc),
    Starter,
    Finisher,
    Multiline(Multiline),
    Goto(Point),
}

impl Entity {
    pub fn start(&self) -> Point {
        match self {
            Self::Line(line) => line.start.clone(),
            Self::Point(point) => point.clone(),
            Self::Arc(arc) => Point {
                x: arc.center.x + arc.radius * arc.start_angle.cos(),
                y: arc.center.y + arc.radius * arc.start_angle.sin(),
                z: arc.center.z,
                layer: arc.layer.clone(),
            },
            Self::Starter => Point::default(),
            Self::Finisher => Point::default(),
            Self::Multiline(multiline) => multiline.start(),
            Self::Goto(goto) => goto.clone(),
        }
    }

    pub fn end(&self) -> Point {
        match self {
            Self::Line(line) => line.end.clone(),
            Self::Point(point) => point.clone(),
            Self::Arc(arc) => Point {
                x: arc.center.x + arc.radius * arc.end_angle.cos(),
                y: arc.center.y + arc.radius * arc.end_angle.sin(),
                z: arc.center.z,
                layer: arc.layer.clone(),
            },
            Self::Starter => Point::default(),
            Self::Finisher => Point::default(),
            Self::Multiline(multiline) => multiline.end(),
            Self::Goto(goto) => goto.clone(),
        }
    }

    pub fn revert(&self) -> Self {
         match self {
            Self::Line(line) => Self::Line(Line {
                start: line.end.clone(),
                end: line.start.clone(),
                layer: line.layer.clone(),
            }),
            Self::Point(point) => Self::Point(point.clone()),
            Self::Arc(arc) => Self::Arc(Arc {
                center: arc.center.clone(),
                radius: arc.radius,
                start_angle: arc.end_angle,
                end_angle: arc.start_angle,
                clockwise: !arc.clockwise,
                layer: arc.layer.clone(),
            }),
            Self::Starter => Self::Starter,
            Self::Finisher => Self::Finisher,
            Self::Multiline(multiline) => {
                let list: Vec<Entity> = multiline.clone().into();
                let reversed = list.iter().rev().map(|e| e.revert()).collect::<Vec<_>>();
                Self::Multiline(Multiline::from(reversed))
            },
            Self::Goto(goto) => Self::Goto(goto.clone()),
        }
    }

    pub fn gcode_path(&self, gcode_options: super::gcode::GCodePathOptions) -> String {
        match self {
            Entity::Line(line) => {
                format!(
                    "{}G{} {}\n",
                    gcode_options.transition_to(&line.start),
                    if gcode_options.feed > 0.0 { "1" } else { "0" },
                    gcode_options.parameters_string(&line.end)
                )
            },
            Entity::Point(point) => {
                format!(
                    "{}G{} {}\n",
                    gcode_options.transition_to(point),
                    if gcode_options.feed > 0.0 { "1" } else { "0" },
                    gcode_options.parameters_string(point)
                )
            },
            Entity::Arc(arc) => {
                arc.gcode_path(gcode_options.clone())
            },
            Entity::Starter => {
                format!("; starting\nG90\nG21\nG0 Z{:.3}\n", gcode_options.security_z)
            },
            Entity::Finisher => {
                format!("; ending\nG0 Z{:.3}\n", gcode_options.security_z)
            },
            Entity::Multiline(multiline) => {
                let starter = gcode_options.transition_to(&self.start());
                let list: Vec<Entity> = multiline.clone().into();
                let output: String = list
                            .iter()
                            .map(|e| e.gcode_path(gcode_options.clone().without_goto_start()))
                            .collect();

                format!("{}{}", starter, output)
            },
            Entity::Goto(goto) => {
                let starter = if gcode_options.goto_start {
                    format!("G0 Z{:.3}\n", gcode_options.security_z)
                } else {
                    "".to_string()
                };

                let mut options = gcode_options.clone();
                options.feed = 0.0;

                format!(
                    "{}G0 {}\n",
                    starter,
                    options.parameters_string(&goto)
                )
            },
        }
    }
}