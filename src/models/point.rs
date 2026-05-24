use super::geometry::Entity;

#[derive(Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Entity for Point {
    fn start(&self) -> Point {
        self.clone()
    }

    fn end(&self) -> Point {
        self.clone()
    }

    fn revert(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }

    fn to_gcode(&self, speed: f64, goto_start: bool) -> String {
        let starter = if goto_start {
            format!(
                "G0 X{:.3} Y{:.3} Z{:.3}\n",
                self.x, self.y, self.z
            )
        } else {
            "".to_string()
        };

        format!(
            "{}G1 X{:.3} Y{:.3} Z{:.3} F{:.1}\n",
            starter, self.x, self.y, self.z, speed
        )
    }
}