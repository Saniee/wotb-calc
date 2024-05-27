use std::collections::HashMap;

use chrono::DateTime;
use strum::IntoEnumIterator;

use crate::data_types::{
    data_info_types::DataInfo,
    tank_data_types::{Nation, Tank, TankType},
};

use crate::misc_funcs::{read_data, update_data};

pub struct App {
    search_term: String,
    selected_nation: Option<Nation>,
    selected_tank_type: Option<TankType>,
    search_result: Vec<Tank>,
    data_info: DataInfo,
    tanks: HashMap<String, Tank>,
    first_tank: Option<Tank>,
    second_tank: Option<Tank>,
}

impl Default for App {
    fn default() -> Self {
        let (data_info, tanks) = read_data();

        Self {
            search_term: Default::default(),
            selected_nation: Default::default(),
            selected_tank_type: Default::default(),
            search_result: Default::default(),
            data_info,
            tanks,
            first_tank: Default::default(),
            second_tank: Default::default(),
        }
    }
}

impl eframe::App for App {
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
                for (_, tank) in self.tanks.clone() {
                    if self.selected_nation.is_some() && self.selected_tank_type.is_some() {
                        if tank.name.contains(&self.search_term)
                            && tank.nation == self.selected_nation.unwrap()
                            && tank.tank_type == self.selected_tank_type.unwrap()
                        {
                            self.search_result.push(tank)
                        }
                    } else if self.selected_nation.is_some() {
                        if tank.name.contains(&self.search_term)
                            && tank.nation == self.selected_nation.unwrap()
                        {
                            self.search_result.push(tank)
                        }
                    } else if self.selected_tank_type.is_some() {
                        if tank.name.contains(&self.search_term)
                            && tank.tank_type == self.selected_tank_type.unwrap()
                        {
                            self.search_result.push(tank)
                        }
                    } else if tank.name.contains(&self.search_term) {
                        self.search_result.push(tank)
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
                            for tank in self.search_result.clone() {
                                if ui.button(tank.name.to_string()).clicked() {
                                    if self.first_tank.is_none() {
                                        self.first_tank = Some(tank)
                                    } else if self.second_tank.is_none() {
                                        self.second_tank = Some(tank)
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
                    if ui.button("Calculate").clicked() {
                        calculate_xp(
                            self.first_tank.clone().unwrap(),
                            self.second_tank.clone().unwrap().tank_id,
                            self.tanks.clone(),
                        );
                    }
                    ui.label("WIP");
                    if ui.button("Clear Selected.").clicked() {
                        self.first_tank = None;
                        self.second_tank = None;
                    }
                }
            });
            ui.add_space(20.0);
            ui.label(
                egui::RichText::new(format!(
                    "Data uses game version: {}\nTanks were last updated at: {}",
                    self.data_info.data.game_version,
                    (DateTime::from_timestamp(self.data_info.data.tanks_updated_at, 0)
                        .unwrap()
                        .to_utc()
                        .format("%d.%m.%Y"))
                ))
                .color(egui::Color32::from_rgb(0, 255, 0)),
            );
            if ui.button("Update Data").clicked() {
                let (data_info, tanks) = update_data();
                self.data_info = data_info;
                self.tanks = tanks;
            }
        });
    }
}

// TODO: Make this shit work.
#[allow(unused)]
fn calculate_xp(
    first_tank: Tank,
    second_tank_id: i64,
    complete_data: HashMap<String, Tank>,
) -> Option<i64> {
    let mut final_value: i64 = 0;
    let mut first_tank_research_candidates: Vec<String> = Vec::new();

    if let Some(next_tanks) = first_tank.next_tanks {
        if next_tanks.len() > 1 {
            first_tank_research_candidates.push(next_tanks.into_iter().next().unwrap().0)
        } else {
            for (id, _) in next_tanks {
                first_tank_research_candidates.push(id)
            }
        }
    }

    if !first_tank_research_candidates.is_empty() {
        Some(final_value)
    } else {
        None
    }

    // println!("{:?}", first_tank_research_candidates)
}
