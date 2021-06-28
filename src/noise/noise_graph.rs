use egui::{*, containers::*, plot::{Line, Values}, widgets::*};
use plot::{
    HLine, Legend, Text, Corner, Plot,
    VLine, Value,
};

use super::noise_generator::NoiseGenerator;


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct NoiseGraph {
    paused: bool,
    selected: i32,


}

impl Default for NoiseGraph {
    fn default() -> Self {
        Self {
            paused: false,
            selected: 1,
        }
    }
}

impl epi::App for NoiseGraph {
    fn name(&self) -> &str {
        "Noise Gourmet"
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default()
            .frame(Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| self.ui(ctx, ui));
      
    }
}

impl NoiseGraph {

    // ??? Draw Ui elements in this function ??? Check update from NoiseGraph  
    pub fn ui(&mut self, ctx: &egui::CtxRef, ui: &mut Ui){

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

            
            let line = Line::new(Values::from_values_iter(ng.sin()));
            let plot = plot.line(line);

            ui.add(plot);

            });            
    }

    // Paramter ui
    fn params_ui(&mut self, ui: &mut Ui){
        ui.checkbox(&mut self.paused, "Paused");

        ComboBox::from_label("Select Noise")
            .selected_text(format!("{:?}", self.selected))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.selected, 1, "First");
                ui.selectable_value(&mut self.selected, 2, "Second");
                ui.selectable_value(&mut self.selected, 3, "Third");
            }
        );

    }



}
