use egui::{*, plot::{Line, Values}};
use plot::{
    HLine, Legend, Text, Corner, Plot,
    VLine, Value,
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
    paused: bool,
    selected: NoiseFunctionNames,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl Default for NoiseGraph {
    fn default() -> Self {
        Self {
            paused: false,
            selected: NoiseFunctionNames::Sin,
            x_min: -10000.,
            x_max: 10000.,
            y_min: -100.,
            y_max: 100.,
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
            .hline(HLine::new(9.0).name("Lines horizontal"))
            .hline(HLine::new(-9.0).name("Lines horizontal"))
            .vline(VLine::new(9.0).name("Lines vertical"))
            .vline(VLine::new(-9.0).name("Lines vertical"))
            .text(Text::new(Value::new(-3.0, -3.0), "wow").name("Text"))
            .text(Text::new(Value::new(-2.0, 2.5), "so graph").name("Text"))
            .text(Text::new(Value::new(3.0, 3.0), "much color").name("Text"))
            .text(Text::new(Value::new(2.5, -2.0), "such plot").name("Text"))
            .legend(Legend::default().position(Corner::RightBottom))
            .show_x(false)
            .show_y(false)
            .data_aspect(1.0);
            
    
            let ng = NoiseGenerator::default();
            
            match self.selected {
                NoiseFunctionNames::Sin => {
                    let line = Line::new(Values::from_values_iter(ng.sin(self.x_min, self.x_max, self.y_min, self.y_max)));
                    let plot = plot.line(line);
                    ui.add(plot);

                }
                NoiseFunctionNames::Cos => {
                    let line = Line::new(Values::from_values_iter(ng.cos(self.x_min, self.x_max, self.y_min, self.y_max)));
                    let plot = plot.line(line);
                    ui.add(plot);

                }
                NoiseFunctionNames::Square => {
                    let line = Line::new(Values::from_values_iter(ng.square(self.x_min, self.x_max, self.y_min, self.y_max)));
                    let plot = plot.line(line);
                    ui.add(plot);
                }

            }

  


            });            
    }

    // Paramter ui
    fn params_ui(&mut self, ui: &mut Ui){
        ui.checkbox(&mut self.paused, "Paused");

        ComboBox::from_label("Select Noise")
            .selected_text(format!("{:?}", self.selected))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.selected, NoiseFunctionNames::Sin, NoiseFunctionNames::Sin.to_string());
                ui.selectable_value(&mut self.selected, NoiseFunctionNames::Cos, NoiseFunctionNames::Cos.to_string());
                ui.selectable_value(&mut self.selected, NoiseFunctionNames::Square, NoiseFunctionNames::Square.to_string());
            }
        );

        ui.add(egui::Slider::new(&mut self.x_min, -100000.0..=0.0).text("X Min"));
        ui.add(egui::Slider::new(&mut self.x_max, 0.0..=100000.0).text("X Max"));


        ui.add(egui::Slider::new(&mut self.y_min, -100.0..=0.0).text("Y Min"));
        ui.add(egui::Slider::new(&mut self.y_max, 0.0..=100.0).text("Y Max"));

    }



}
