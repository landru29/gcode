#[derive(Clone)]
pub struct GCodeOptions{
    pub feed: f64,
    pub security_z: f64,
    pub goto_start: bool,
    pub with_x: bool,
    pub with_y: bool,
    pub with_z: bool,
    pub arc_center: Option<(f64, f64)>,
}

type Configurator = Box<dyn Fn(&mut GCodeOptions)>;

impl GCodeOptions {
    pub fn new(configurators: &Vec<Configurator>) -> Self {
        let mut options = Self {
            feed: 0.0,
            goto_start: false,
            with_x: false,
            with_y: false,
            with_z: false,
            security_z: 0.0,
            arc_center: None,
        };
        for configurator in configurators {
            configurator(&mut options);
        }
        options
    }

    pub fn with_arc_center(x: &f64, y: &f64) -> Configurator {
        let x = x.clone();
        let y = y.clone();
        Box::new(move |options: &mut Self| {
            options.arc_center = Some((x, y));
        })
    }

    pub fn with_feed(feed: &f64) -> Configurator {
        let feed = feed.clone();
        Box::new(move |options: &mut Self| {
            options.feed = feed;
        })
    }

    pub fn with_security_z(security_z: &f64) -> Configurator {
        let security_z = security_z.clone();
        Box::new(move |options: &mut Self| {
            options.security_z = security_z;
        })
    }

    pub fn with_x() -> Configurator {
        Box::new(|options: &mut Self| {
            options.with_x = true;
        })
    }

    pub fn with_y() -> Configurator {
        Box::new(|options: &mut Self| {
            options.with_y = true;
        })
    }

    pub fn with_z() -> Configurator {
        Box::new(|options: &mut Self| {
            options.with_z = true;
        })
    }

    pub fn with_goto_start() -> Configurator {
        Box::new(|options: &mut Self| {
            options.goto_start = true;
        })
    }

    pub fn transition_to(&self, point: &super::point::Point) -> String {
        if !self.goto_start {
            return "".to_string();
        }

        format!("G0 Z{:.3}\nG0 X{:.3} Y{:.3}\nG1 Z{:.3} F{:.1}", self.security_z, point.x, point.y, point.z, self.feed)
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

        match self.arc_center {
            Some((i, j)) => {
                vec![
                    format!("I{:.3}", i), 
                    format!("J{:.3}", j),
                ].iter().for_each(|param| params.push(param.clone()));
            },
            None => {}
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