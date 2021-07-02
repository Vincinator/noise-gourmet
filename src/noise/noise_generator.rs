
use egui::plot::Value;



pub struct NoiseGenerator {
}

impl Default for NoiseGenerator {
    fn default() -> Self {
        Self {
        }
    }
}


impl NoiseGenerator {

    pub fn sin(self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> impl Iterator<Item = Value>{
        (x_min as i64..x_max as i64).map(move |i| {
            let x = i as f64 * 0.01;

            let mut y = x.sin();
            
            if y > y_max {
                y = y_max;
            }

            if y < y_min {
                y = y_min;
            }

            Value::new(x, y)
        })
    }

    pub fn cos(self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> impl Iterator<Item = Value>{
        (x_min as i64..x_max as i64).map(move |i| {
            let x = i as f64 * 0.01;
            let mut y = x.sin();
            
            if y > y_max {
                y = y_max;
            }

            if y < y_min {
                y = y_min;
            }

            Value::new(x, y)
        })
    }

    pub fn square(self, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> impl Iterator<Item = Value>{
        (x_min as i64..x_max as i64).map(move |i| {
            let x = i as f64 * 0.01;
            let mut y = x *x;
            if y > y_max {
                y = y_max;
            }

            if y < y_min {
                y = y_min;
            }

            Value::new(x, y) 
        })
    }

}