#![allow(dead_code)]

use std::collections::HashMap;

use egui::Ui;
use strum::IntoEnumIterator;

use crate::data_types::tank_data_types::{Nation, Tank, TankType};
use crate::pathfinding::ResearchPath;

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

pub fn found_results(
    ui: &mut Ui,
    found_results: &mut Vec<Tank>,
    tanks: (&mut Option<Tank>, &mut Option<Tank>),
) {
    egui::ScrollArea::vertical()
        .id_source("search_results")
        .auto_shrink(false)
        .max_height(200.0)
        .show(ui, |ui| {
            let (first_tank, second_tank) = tanks;
            if !found_results.is_empty() {
                for tank in found_results {
                    if ui.button(tank.name.to_string()).clicked() {
                        tank_selection((first_tank, second_tank), tank.clone())
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

pub fn research_result(ui: &mut Ui, path: &Option<ResearchPath>) {
    let path = match path {
        None => return,
        Some(p) => p,
    };

    if path.steps.is_empty() {
        ui.label(
            egui::RichText::new("Same tank selected  - no research needed.")
                .color(egui::Color32::from_rgb(180, 180, 0)),
        );
        return;
    }

    ui.label(
        egui::RichText::new(format!("Total XP needed: {}", format_xp(path.total_xp)))
            .size(16.0)
            .strong()
            .color(egui::Color32::from_rgb(255, 200, 0)),
    );
    ui.add_space(6.0);

    egui::ScrollArea::vertical()
        .id_source("research_result")
        .max_height(300.0)
        .auto_shrink([false, true])
        .show(ui, |ui| {
            for step in &path.steps {
                ui.group(|ui| {
                    ui.label(
                        egui::RichText::new(format!("On: {}", step.tank_name))
                            .strong()
                            .color(egui::Color32::from_rgb(100, 200, 255)),
                    );
                    if step.modules_to_research.is_empty() {
                        ui.label("  (gate module already unlocked)");
                    } else {
                        for m in &step.modules_to_research {
                            ui.label(format!("  * Research {}  - {} XP", m.module_name, format_xp(m.xp_cost)));
                        }
                    }
                    ui.label(
                        egui::RichText::new(format!(
                            "  -> Unlock {}  - {} XP",
                            step.next_tank_name,
                            format_xp(step.next_tank_xp)
                        ))
                        .color(egui::Color32::from_rgb(0, 220, 100)),
                    );
                });
                ui.add_space(4.0);
            }
        });
}

fn format_xp(xp: i64) -> String {
    if xp >= 1_000_000 {
        format!("{:.1}M", xp as f64 / 1_000_000.0)
    } else if xp >= 1_000 {
        let s = xp.to_string();
        let (h, t) = s.split_at(s.len() - 3);
        if h.is_empty() { format!("{}", t) } else { format!("{},{}", h, t) }
    } else {
        xp.to_string()
    }
}
