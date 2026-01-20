use crate::main_view::MirroredData;
use eframe::egui;



pub struct OptionsView{
    pub selected_algo: String,
}

impl OptionsView{
    pub fn new() -> Self {
        Self {
            selected_algo: "CgrFirstEndingContactParenting".to_string(),
        }
    }
    pub fn show(&mut self,ui: &mut egui::Ui, _data: &mut MirroredData){
        ui.vertical(|ui| {
            ui.add_space(5.0);
            ui.heading("Options");
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(15.0);

            //Contact Plan
            ui.horizontal(|ui| {
                ui.label("Change Contact Plan :");
                if ui.button("Explore").clicked() {
                    // Action future ici
                }
            });

            ui.add_space(10.0);

            //Algorithme
            ui.horizontal(|ui| {
                ui.label("Change Algo :");
                
                egui::ComboBox::from_id_salt("algo_dropdown")
                    .selected_text(&self.selected_algo)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_algo, "CgrFirstEndingHybridParenting".to_string(), "CgrFirstEndingHybridParenting");
                        ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedHybridParenting".to_string(), "CgrFirstDepletedHybridParenting");
                        ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedNodeParenting".to_string(), "CgrFirstDepletedNodeParenting");
                        ui.selectable_value(&mut self.selected_algo, "CgrFirstEndingContactParenting".to_string(), "CgrFirstEndingContactParenting",);
                        ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedContactParenting".to_string(), "CgrFirstDepletedContactParenting");
                    });
            });
        });
    }
}