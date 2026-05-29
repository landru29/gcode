#[derive(Clone, Default)]
pub struct GCodePathOptions{
    pub feed: f64,
    pub security_z: Option<f64>,
    pub goto_start: bool,
    pub with_x: bool,
    pub with_y: bool,
    pub with_z: bool,
    pub override_z: Option<f64>
}

impl GCodePathOptions {
    pub fn with_feed(mut self, feed: f64) -> Self {
        self.feed = feed;
        self
    }

    pub fn with_security_z(mut self, security_z: f64) -> Self {
        self.security_z = Some(security_z);
        self
    }

    pub fn without_security_z(mut self) -> Self {
        self.security_z = None;
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

    pub fn with_override_z(mut self, z: f64) -> Self {
        self.override_z = Some(z);
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

        let security = match self.security_z {
            Some(z) => format!("G0 Z{:.3} ;security\n", z),
            _ => String::from(""),
        };

        format!("{}G0 X{:.3} Y{:.3} ;transition\n", security, point.x, point.y)
    }


    pub fn parameters_string(&self, point: &super::point::Point) -> String {
        let mut params:Vec<String> = vec![];


        if self.with_x && !self.goto_start {
            params.push(format!("X{:.3}", point.x));
        }

        if self.with_y && !self.goto_start {
            params.push(format!("Y{:.3}", point.y));
        }

        if self.with_z {
            match self.override_z {
                Some(z) => params.push(format!("Z{:.3}", z)),
                _ => params.push(format!("Z{:.3}", point.z)),
            };
        }

        if self.feed > 0.0 {
            params.push(format!("F{:.1}", self.feed));
        }

        params.join(" ").to_string()
    }

    pub fn optional_security(&self) -> String {
        self.security_z.map_or(String::from(""), |z| format!("G0 Z{:.3} ; security\n", z))
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