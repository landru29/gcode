use crate::geometry::{
    geometry::Entity, 
    point::Point,
};

#[derive(Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

impl Entity for Line {
    fn start(&self) -> Point {
        self.start.clone()
    }

    fn end(&self) -> Point {
        self.end.clone()
    }

    fn revert(&self) -> Box<dyn Entity> {
        Box::new(Self {
            start: self.end.clone(),
            end: self.start.clone(),
        })
    }

    fn to_gcode(&self, speed: f64, goto_start: bool) -> String {
        let starter = if goto_start {
            format!(
                "G0 X{:.3} Y{:.3} Z{:.3}\n",
                self.start.x, self.start.y, self.start.z
            )
        } else {
            "".to_string()
        };
        
        format!(
            "{}G1 X{:.3} Y{:.3} Z{:.3} F{:.1}\n",
            starter, self.end.x, self.end.y, self.end.z, speed
        )
    }
}