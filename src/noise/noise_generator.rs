
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


    pub fn map_fun(self, f:  Box<dyn Fn(f64) -> f64>, x_min: f64, x_max: f64, y_min: f64, y_max: f64, resolution: f64) -> impl Iterator<Item = Value> {

        ((x_min*resolution) as i64..(x_max*resolution) as i64).map(move |i| {
            let x = i as f64 / resolution ;

            let mut y = f(x);
            
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