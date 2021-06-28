use std::collections::btree_map::VacantEntry;

use egui::plot::Value;



pub struct NoiseGenerator {
    seed: i32,
}

impl Default for NoiseGenerator {
    fn default() -> Self {
        Self {
            seed: 1252151,
        }
    }
}


impl NoiseGenerator {


    pub fn sin(self) -> impl Iterator<Item = Value>{
        let sin = (-1000..1000).map(|i| {
            let x = i as f64 * 0.01;
            Value::new(x, x.sin())
        });
        
        return sin;
    }

}