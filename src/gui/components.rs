// TODO: To be made.
#![allow(dead_code)]

use std::collections::HashMap;

use egui::Ui;
use strum::IntoEnumIterator;

use crate::data_types::tank_data_types::{Nation, Tank, TankType};

use super::main_funcs::{search_database, tank_selection};

#[allow(unused)]
pub fn search(
    ui: &mut Ui,
    filters: (String, Option<Nation>, Option<TankType>, bool),
    database: HashMap<String, Tank>,
    search_result: &mut Vec<Tank>,
) {
    if ui.button("Search").clicked() {
        *search_result = Vec::new();
        search_database(filters, database, search_result);
    }
}

pub fn filters(
    ui: &mut Ui,
    selected_nation: &mut Option<Nation>,
    selected_tank_type: &mut Option<TankType>,
    ignore_case: &mut bool,
) {
    ui.vertical_centered(|ui| {
        egui::ComboBox::from_label("Tank Nation")
            .selected_text(format!("{:?}", selected_nation))
            .show_ui(ui, |ui| {
                ui.selectable_value(selected_nation, None, "None");
                for nation in Nation::iter() {
                    ui.selectable_value(selected_nation, Some(nation), format!("{:?}", nation));
                }
            });
        egui::ComboBox::from_label("Tank Type")
            .selected_text(format!("{:?}", selected_tank_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(selected_tank_type, None, "None");
                for tank_type in TankType::iter() {
                    ui.selectable_value(
                        selected_tank_type,
                        Some(tank_type),
                        format!("{:?}", tank_type),
                    );
                }
            });

        ui.checkbox(ignore_case, "Ignore Upper/Lower Case?");
    });
}

pub fn found_results(ui: &mut Ui, found_results: &mut Vec<Tank>) {
    egui::ScrollArea::vertical()
        .auto_shrink(false)
        .max_height(200.0)
        .show(ui, |ui| {
            if !found_results.is_empty() {
                for tank in found_results {
                    if ui.button(tank.name.to_string()).clicked() {
                        tank_selection()
                    }
                }
            } else {
                ui.label(
                    egui::RichText::new("Search for a tank first!")
                        .color(egui::Color32::from_rgb(255, 0, 0))
                        .size(15.0),
                );
            }
        });
}
