use egui::plot::Value;

#[derive(Clone, Copy)]
pub struct NoiseGenerator {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub resolution: f64,
}

impl Default for NoiseGenerator {
    fn default() -> Self {
        Self {
            x_min: -20.,
            x_max: 20.,
            y_min: -10.,
            y_max: 10.,
            resolution: 1.,
        }
    }
}

impl NoiseGenerator {
    pub fn generate_points(self, f: Box<dyn Fn(f64) -> f64>) -> Vec<Value> {
        let mut values = Vec::<Value>::new();
        for i in (self.x_min * self.resolution) as i64..(self.x_max * self.resolution) as i64 {
            let x = i as f64 / self.resolution;

            let mut y = f(x);

            if y > self.y_max {
                y = self.y_max;
            }

            if y < self.y_min {
                y = self.y_min;
            }
            values.push(Value::new(x, y));
        }
        values
    }
}
