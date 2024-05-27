use std::{collections::HashMap, fs};

use strum::IntoEnumIterator;

use crate::tank_data_types::{Nation, Tank, TankData, TankType};

pub struct MyApp {
    search_term: String,
    selected_nation: Option<Nation>,
    selected_tank_type: Option<TankType>,
    search_result: Vec<Tank>,
    tanks: HashMap<String, Tank>,
    first_tank: Option<Tank>,
    second_tank: Option<Tank>,
}

impl Default for MyApp {
    fn default() -> Self {
        let tanks = read_data();

        Self {
            search_term: Default::default(),
            selected_nation: Default::default(),
            selected_tank_type: Default::default(),
            search_result: Default::default(),
            tanks,
            first_tank: Default::default(),
            second_tank: Default::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let label = ui.label("Search");
            ui.text_edit_singleline(&mut self.search_term)
                .labelled_by(label.id);
            egui::ComboBox::from_label("Select Nation")
                .selected_text(format!("{:?}", self.selected_nation))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_nation, None, "None");
                    for nation in Nation::iter() {
                        ui.selectable_value(
                            &mut self.selected_nation,
                            Some(nation),
                            format!("{:?}", nation),
                        );
                    }
                });
            egui::ComboBox::from_label("Select Tank Type")
                .selected_text(format!("{:?}", self.selected_tank_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_tank_type, None, "None");
                    for tank_type in TankType::iter() {
                        ui.selectable_value(
                            &mut self.selected_tank_type,
                            Some(tank_type),
                            format!("{:?}", tank_type),
                        );
                    }
                });
            if ui.button("Search").clicked() {
                self.search_result = Vec::new();
                for (_, tank) in self.tanks.iter() {
                    if self.selected_nation.is_some() && self.selected_tank_type.is_some() {
                        if tank.name.contains(&self.search_term)
                            && tank.nation == self.selected_nation.unwrap()
                            && tank.tank_type == self.selected_tank_type.unwrap()
                        {
                            self.search_result.push(tank.clone())
                        }
                    } else if self.selected_nation.is_some() {
                        if tank.name.contains(&self.search_term)
                            && tank.nation == self.selected_nation.unwrap()
                        {
                            self.search_result.push(tank.clone())
                        }
                    } else if self.selected_tank_type.is_some() {
                        if tank.name.contains(&self.search_term)
                            && tank.tank_type == self.selected_tank_type.unwrap()
                        {
                            self.search_result.push(tank.clone())
                        }
                    } else if tank.name.contains(&self.search_term) {
                        self.search_result.push(tank.clone())
                    }
                }
            }

            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Click on two to calculate the XP needed.").size(15.0),
                );
                if (self.first_tank.is_some() && self.second_tank.is_none())
                    || (self.first_tank.is_none() && self.second_tank.is_some())
                {
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("Click on one more tank to get the calculation!")
                            .size(12.5)
                            .color(egui::Color32::from_rgb(0, 255, 0)),
                    );
                    ui.add_space(10.0);
                }

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .auto_shrink(false)
                    .scroll_bar_visibility(
                        egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                    )
                    .show(ui, |ui| {
                        if !self.search_result.is_empty() {
                            for tank in self.search_result.iter() {
                                if ui.button(tank.name.to_string()).clicked() {
                                    if self.first_tank.is_none() {
                                        self.first_tank = Some(tank.clone())
                                    } else if self.second_tank.is_none() {
                                        self.second_tank = Some(tank.clone())
                                    }
                                }
                            }
                        }
                    });
            });

            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new("Calculated Free XP Required:").size(15.0));
                if self.first_tank.is_some() && self.second_tank.is_some() {
                    ui.label("WIP");
                    if ui.button("Clear Selected.").clicked() {
                        self.first_tank = None;
                        self.second_tank = None;
                    }
                }
            })
        });
    }
}

fn read_data() -> HashMap<String, Tank> {
    let path = std::path::Path::new("./src/tank_data.json");
    let str_data = fs::read_to_string(path).expect("Couldn't read the data file.");

    let data: TankData = serde_json::from_str(&str_data).expect("Error reading data!");

    data.tanks
}

#[allow(dead_code)]
#[allow(unused)]
fn calculate_xp(first_tank: Tank, second_tank: Tank, complete_data: HashMap<String, Tank>) {
    for (id, tank) in complete_data {
        for tank in first_tank.next_tanks.clone().iter() {}
    }
}
