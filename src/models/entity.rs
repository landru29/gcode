use core::fmt;

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
    Comment(String),
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
            Self::Comment(_) => Point::default(),
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
            Self::Comment(_) => Point::default(),
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
            Self::Comment(s) => Self::Comment(s.clone()),
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
            Self::Line(line) => {
                format!(
                    "; {}\n{}G{} {}\n",
                    line,
                    gcode_options.transition_to(&line.start),
                    if gcode_options.feed > 0.0 { "1" } else { "0" },
                    gcode_options.parameters_string(&line.end)
                )
            },
            Self::Point(point) => {
                format!(
                    "; {}\n{}G{} {}\n",
                    point,
                    gcode_options.transition_to(point),
                    if gcode_options.feed > 0.0 { "1" } else { "0" },
                    gcode_options.parameters_string(point)
                )
            },
            Self::Arc(arc) => {
                arc.gcode_path(gcode_options.clone())
            },
            Self::Starter => {
                format!("; starting\nG90 ; Absolute coordinates\nG21 ; millimeters\n{}", gcode_options.optional_security())
            },
            Self::Finisher => {
                format!("; ending\n{}G0 X0.000 Y0.000\n", gcode_options.optional_security())
            },
            Self::Multiline(multiline) => {
                let starter = gcode_options.transition_to(&self.start());
                let list: Vec<Self> = multiline.clone().into();
                let output: String = list
                            .iter()
                            .map(|e| e.gcode_path(gcode_options.clone().without_goto_start().without_security_z()))
                            .collect();

                format!("; {}\n{}{}{}", multiline, starter, output, gcode_options.optional_security())
            },
            Self::Goto(goto) => {
                let starter = if gcode_options.goto_start {
                    format!("{}", gcode_options.optional_security())
                } else {
                    "".to_string()
                };

                let mut options = gcode_options.clone();
                options.feed = 0.0;

                format!(
                    "; Goto {}\n{}G0 {}\n",
                    goto,
                    starter,
                    options.parameters_string(&goto)
                )
            },
            Self::Comment(s) => {
                s.split('\n').map(|line| format!("; {}\n", line)).collect()
            },
        }
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Line(line) => write!(f, "{}", line),
            Self::Point(point) => write!(f, "{}", point),
            Self::Arc(arc) => write!(f, "{}", arc),
            Self::Starter => write!(f, "starter"),
            Self::Finisher => write!(f, "finisher"),
            Self::Multiline(multiline) => write!(f, "{}", multiline),
            Self::Goto(goto) => write!(f, "{}", goto),
            Self::Comment(s) => write!(f, "{}", s),
        }
    }
}