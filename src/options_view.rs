use crate::main_view::MirroredData;
use eframe::egui;
use egui_file_dialog::FileDialog;


pub struct OptionsView{
    pub selected_algo: String,
    pub selected_cp_path: String,
    file_dialog: FileDialog,
}

impl OptionsView{
    pub fn new() -> Self {
        Self {
            selected_algo: "VolCgrNodeParenting".to_string(),
            selected_cp_path: "./db/ion.cp".to_string(),
            file_dialog: FileDialog::new(),
        }
    }
    pub fn show(&mut self,ui: &mut egui::Ui, _data: &mut MirroredData) -> Option<(String,String)>{
        self.file_dialog.update(ui.ctx());
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
                if ui.button("🛰  Explore").clicked() {
                    self.file_dialog.pick_file();
                    }
            });

            if let Some(path) = self.file_dialog.take_picked(){
                self.selected_cp_path = path.display().to_string();
                update = true;
            }

            ui.add_space(10.0);

            //Algorithme
            ui.horizontal(|ui| {
                ui.label("Change Algo :");
                let old_algo = self.selected_algo.clone();
                
                egui::ComboBox::from_id_salt("algo_selector")
                    .selected_text(&self.selected_algo)
                    .show_ui(ui, |ui| {
                        //No features
                        ui.selectable_value(&mut self.selected_algo, "VolCgrNodeParenting".to_string(), "VolCgrNodeParenting");
                        ui.selectable_value(&mut self.selected_algo, "VolCgrHybridParenting".to_string(), "VolCgrHybridParenting");
                        ui.selectable_value(&mut self.selected_algo, "VolCgrHybridParentingHop".to_string(), "VolCgrHybridParentingHop");
                        ui.selectable_value(&mut self.selected_algo, "VolCgrNodeParentingHop".to_string(), "VolCgrNodeParentingHop");
                        #[cfg(feature = "contact_work_area")]{
                            ui.selectable_value(&mut self.selected_algo, "VolCgrContactParenting".to_string(), "VolCgrContactParenting");
                            ui.selectable_value(&mut self.selected_algo, "VolCgrContactParentingHop".to_string(), "VolCgrContactParentingHop");
                        }
                        #[cfg(feature = "contact_suppression")]{
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstEndingHybridParentingHop".to_string(), "CgrFirstEndingHybridParentingHop");
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstEndingHybridParenting".to_string(), "CgrFirstEndingHybridParenting");
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstEndingNodeParentingHop".to_string(), "CgrFirstEndingNodeParentingHop");
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstEndingNodeParenting".to_string(), "CgrFirstEndingNodeParenting");
                        }
                        #[cfg(all(feature = "contact_work_area", feature = "contact_suppression"))]{
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstEndingContactParenting".to_string(), "CgrFirstEndingContactParenting");
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstEndingHybridParenting".to_string(), "CgrFirstEndingHybridParenting");
                        }
                        #[cfg(all(feature = "contact_suppression", feature = "first_depleted"))]{
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedHybridParentingHop".to_string(), "CgrFirstDepletedHybridParentingHop");
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedHybridParenting".to_string(), "CgrFirstDepletedHybridParenting");
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedNodeParentingHop".to_string(), "CgrFirstDepletedNodeParentingHop");
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedNodeParenting".to_string(), "CgrFirstDepletedNodeParenting");
                        }
                        #[cfg(all(feature = "contact_work_area",feature = "contact_suppression",feature = "first_depleted"))]{
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedContactParentingHop".to_string(), "CgrFirstDepletedContactParentingHop");
                            ui.selectable_value(&mut self.selected_algo, "CgrFirstDepletedContactParenting".to_string(), "CgrFirstDepletedContactParenting");
                        }
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