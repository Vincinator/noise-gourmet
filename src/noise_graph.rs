use egui::{containers::*, widgets::*, *};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct NoiseGraph {


}

impl Default for NoiseGraph {
    fn default() -> Self {
        Self {
 
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
            .show(ctx, |ui| self.ui(ui));
      
    }
}

impl NoiseGraph {

    pub fn ui(&mut self, ui: &mut Ui){

        let painter = Painter::new(
            ui.ctx().clone(),
            ui.layer_id(),
            ui.available_rect_before_wrap_finite(),
        );

        self.paint(&painter);

        ui.expand_to_include_rect(painter.clip_rect());

        Frame::popup(ui.style())
            .stroke(Stroke::none())
            .show(ui, |ui| {
                ui.set_max_width(320.0);
                CollapsingHeader::new("Parameter")
                    .show(ui, |ui| self.options_ui(ui));
            });
            
    }

    fn options_ui(&mut self, ui: &mut Ui){

    }


    fn paint(&mut self, painter: &Painter){

    }



}
