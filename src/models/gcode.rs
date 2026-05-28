#[derive(Clone, Default)]
pub struct GCodePathOptions{
    pub feed: f64,
    pub security_z: f64,
    pub goto_start: bool,
    pub with_x: bool,
    pub with_y: bool,
    pub with_z: bool,
}

impl GCodePathOptions {
    pub fn with_feed(mut self, feed: f64) -> Self {
        self.feed = feed;
        self
    }

    pub fn with_security_z(mut self, security_z: f64) -> Self {
        self.security_z = security_z;
        self
    }

    pub fn with_x(mut self) -> Self {
        self.with_x = true;
        self
    }

    pub fn with_y(mut self) -> Self {
        self.with_y = true;
        self
    }

    pub fn with_z(mut self) -> Self {
        self.with_z = true;
        self
    }

    pub fn with_goto_start(mut self) -> Self {
        self.goto_start = true;
        self
    }

    pub fn without_goto_start(mut self) -> Self {
        self.goto_start = false;
        self
    }

    pub fn transition_to(&self, point: &super::point::Point) -> String {
        if !self.goto_start {
            return "".to_string();
        }

        format!("G0 Z{:.3}\nG0 X{:.3} Y{:.3}\nG1 Z{:.3} F{:.1}\n", self.security_z, point.x, point.y, point.z, self.feed)
    }


    pub fn parameters_string(&self, point: &super::point::Point) -> String {
        let mut params:Vec<String> = vec![];

        if self.with_x {
            params.push(format!("X{:.3}", point.x));
        }

        if self.with_y {
            params.push(format!("Y{:.3}", point.y));
        }

        if self.with_z {
            params.push(format!("Z{:.3}", point.z));
        }

        if self.feed > 0.0 {
            params.push(format!("F{:.1}", self.feed));
        }

        params.join(" ").to_string()
    }
}


// Utility function to generate an array of steps from 0 to deep with a given step size
pub fn step_array(deep: f64, step: f64) -> Vec<f64> {
    let mut steps = vec![];
    let mut current = step;

    while current < deep {
        steps.push(current);
        current += step;
    }

    steps.push(deep);

    steps
}