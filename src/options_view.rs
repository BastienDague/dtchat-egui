use crate::main_view::MirroredData;
use eframe::egui;
use rfd::FileDialog;


pub struct OptionsView{
    pub selected_algo: String,
    pub selected_cp_path: String,
}

impl OptionsView{
    pub fn new() -> Self {
        Self {
            selected_algo: "CgrFirstEndingContactParenting".to_string(),
            selected_cp_path: "./db/ion.cp".to_string(),
        }
    }
    pub fn show(&mut self,ui: &mut egui::Ui, _data: &mut MirroredData) -> Option<(String,String)>{
        let mut update = false;
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
                    if let Some(path) = FileDialog::new()
                    .set_directory("/")
                    .pick_file(){
                        self.selected_cp_path = path.display().to_string();
                        update = true;
                    }
                }
            });

            ui.add_space(10.0);

            //Algorithme
            ui.horizontal(|ui| {
                ui.label("Change Algo :");
                let old_algo = self.selected_algo.clone();
                
                egui::ComboBox::from_id_salt("algo_selector")
                    .selected_text(&self.selected_algo)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected_algo, "CgrFirstEndingContactParenting".to_string(), "CgrFirstEndingContactParenting");
                        ui.selectable_value(&mut self.selected_algo, "CgrFirstEndingHybridParenting".to_string(), "CgrFirstEndingHybridParenting");
                        // Crash with others algos
                        //ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedHybridParenting".to_string(), "CgrFirstDepletedHybridParenting");
                        //ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedNodeParenting".to_string(), "CgrFirstDepletedNodeParenting",);
                        //ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedContactParenting".to_string(), "CgrFirstDepletedContactParenting");
                    });
                if old_algo != self.selected_algo{
                    update = true;
                }
            });
        });
        if update{
            Some((self.selected_cp_path.clone(),self.selected_algo.clone()))
        } else{
            None
        }
    }
}