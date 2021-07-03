use egui::{*, plot::{Line, Values}};
use plot::{
    HLine, Legend, Corner, Plot,
    VLine, 
};

use super::noise_generator::NoiseGenerator;
#[derive(PartialEq,Debug)]
enum NoiseFunctionNames {
    Sin = 0,
    Cos = 1,
    Square = 2,
}

impl ToString for NoiseFunctionNames {
    fn to_string(&self) -> std::string::String{
        match self{
            NoiseFunctionNames::Sin => String::from("Sin"),
            NoiseFunctionNames::Cos => String::from("Cos"),
            NoiseFunctionNames::Square => String::from("Square"),
        }

    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct NoiseGraph {
    selected: NoiseFunctionNames,
    ng: NoiseGenerator,
    show_maxima: bool,
}

impl Default for NoiseGraph {
    fn default() -> Self {
        Self {
            selected: NoiseFunctionNames::Sin,

            ng: NoiseGenerator::default(),
            show_maxima: true,

        }
    }
}

impl epi::App for NoiseGraph {
    fn name(&self) -> &str {
        "Noise Gourmet"
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| self.ui(ctx, ui));
      
    }
}

impl NoiseGraph {

    // ??? Draw Ui elements in this function ??? Check update from NoiseGraph  
    pub fn ui(&mut self, ctx: &egui::CtxRef, _ui: &mut Ui){

        egui::SidePanel::left("params_panel")
            .show(ctx, |ui| self.params_ui(ui));
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("Items Demo")
            .legend(Legend::default().position(Corner::RightBottom))
            .show_x(false)
            .show_y(false)
            .data_aspect(1.0);
            
    

            let limit_y_max = HLine::new(self.ng.y_max);
            let limit_y_min = HLine::new(self.ng.y_min);
            let limit_x_max = VLine::new(self.ng.x_max);
            let limit_x_min = VLine::new(self.ng.x_min);
            let line;

            match self.selected {
                NoiseFunctionNames::Sin => {
                    let values = self.ng.generate_points( ng_sin());
                   
                    line = Line::new(Values::from_values(values));
                }
                NoiseFunctionNames::Cos => {
                    let values = self.ng.generate_points(ng_cos());


                    line = Line::new(Values::from_values(values));
                }
                NoiseFunctionNames::Square => {
                    let values = self.ng.generate_points( ng_square());

                    line = Line::new(Values::from_values(values));
                }
            }    
                let plot = plot.line(line)
                    .hline(limit_y_min)
                    .hline(limit_y_max)
                    .vline(limit_x_max)
                    .vline(limit_x_min);
                    ui.add(plot);
        
        });    
                  
    }

    // Paramter ui
    fn params_ui(&mut self, ui: &mut Ui){

        ComboBox::from_label("Select Noise")
            .selected_text(format!("{:?}", self.selected))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.selected, NoiseFunctionNames::Sin, NoiseFunctionNames::Sin.to_string());
                ui.selectable_value(&mut self.selected, NoiseFunctionNames::Cos, NoiseFunctionNames::Cos.to_string());
                ui.selectable_value(&mut self.selected, NoiseFunctionNames::Square, NoiseFunctionNames::Square.to_string());
            }
        );
        ui.add(egui::Separator::default());
        ui.checkbox(&mut self.show_maxima, "Show Maxima");
        ui.add(egui::Separator::default());


        ui.add(egui::Slider::new(&mut self.ng.x_min, -100.0..=0.0).text("X Min"));
        ui.add(egui::Slider::new(&mut self.ng.x_max, 0.0..=100.0).text("X Max"));


        ui.add(egui::Slider::new(&mut self.ng.y_min, -100.0..=0.0).text("Y Min"));
        ui.add(egui::Slider::new(&mut self.ng.y_max, 0.0..=100.0).text("Y Max"));
        ui.add(egui::Separator::default());
        ui.add(egui::Slider::new(&mut self.ng.resolution, 1.0..=100.0).text("Resolution"));

    }

}



pub fn ng_sin() -> Box<dyn Fn(f64) -> f64>{
    Box::new(|x: f64| x.sin())
}

pub fn ng_cos() -> Box<dyn Fn(f64) -> f64>{
    Box::new(|x: f64| x.cos())
}
pub fn ng_square() -> Box<dyn Fn(f64) -> f64>{
    Box::new(|x: f64| x*x)
}