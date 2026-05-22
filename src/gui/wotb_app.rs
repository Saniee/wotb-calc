use std::collections::HashMap;

use chrono::DateTime;
use egui::RichText;

use crate::{
    data_types::{
        data_info_types::DataInfo,
        tank_data_types::{Nation, Tank, TankType},
    },
    misc_funcs::update_data,
    pathfinding::ResearchPath,
};

use crate::misc_funcs::read_data;

use super::components::{filters, found_results, research_result, search};
use super::main_funcs::calculate_xp;

#[allow(unused)]
pub struct WotbApp {
    search_term: String,
    selected_nation: Option<Nation>,
    selected_tank_type: Option<TankType>,
    ignore_case: bool,
    search_result: Vec<Tank>,
    data_info: DataInfo,
    tanks: HashMap<String, Tank>,
    first_tank: Option<Tank>,
    second_tank: Option<Tank>,
    research_path: Option<ResearchPath>,
}

impl Default for WotbApp {
    fn default() -> Self {
        let (data_info, tanks) = read_data();

        Self {
            search_term: Default::default(),
            selected_nation: Default::default(),
            selected_tank_type: Default::default(),
            search_result: Default::default(),
            ignore_case: true,
            data_info,
            tanks,
            first_tank: Default::default(),
            second_tank: Default::default(),
            research_path: None,
        }
    }
}

impl eframe::App for WotbApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
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
                    self.research_path = None;
                }
                ui.add_space(12.5);
                ui.label(
                    egui::RichText::new("World of Tanks Blitz Tools")
                        .color(egui::Color32::from_rgb(0, 255, 0))
                        .strong()
                        .size(20.0),
                );
                search(
                    ui,
                    (
                        self.search_term.clone(),
                        self.selected_nation,
                        self.selected_tank_type,
                        self.ignore_case,
                    ),
                    self.tanks.clone(),
                    &mut self.search_result,
                );
                ui.add(egui::TextEdit::singleline(&mut self.search_term));
                filters(
                    ui,
                    &mut self.selected_nation,
                    &mut self.selected_tank_type,
                    &mut self.ignore_case,
                );
                ui.add_space(15.0);
                let prev_first = self.first_tank.as_ref().map(|t| t.tank_id);
                let prev_second = self.second_tank.as_ref().map(|t| t.tank_id);
                found_results(
                    ui,
                    &mut self.search_result,
                    (&mut self.first_tank, &mut self.second_tank),
                );
                let new_first = self.first_tank.as_ref().map(|t| t.tank_id);
                let new_second = self.second_tank.as_ref().map(|t| t.tank_id);
                if prev_first != new_first || prev_second != new_second {
                    self.research_path = None;
                }

                ui.add_space(10.0);
                match (&self.first_tank, &self.second_tank) {
                    (Some(a), Some(b)) => {
                        ui.label(
                            RichText::new(format!("Selected: {} -> {}", a.name, b.name))
                                .size(14.0)
                                .color(egui::Color32::from_rgb(0, 125, 255)),
                        );
                        if ui.button("Calculate Path").clicked() {
                            self.research_path =
                                calculate_xp(&self.tanks, a, b);
                        }
                    }
                    (Some(a), None) => {
                        ui.label(
                            RichText::new(format!("From: {}  - pick a second tank", a.name))
                                .size(14.0)
                                .color(egui::Color32::from_rgb(200, 200, 0)),
                        );
                    }
                    _ => {}
                }
                ui.add_space(10.0);
                research_result(ui, &self.research_path);
            })
        });
    }
}
