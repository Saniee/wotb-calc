use crate::gui::wotb_app::WotbApp;

mod data_types;
mod gui;
mod misc_funcs;
mod pathfinding;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 750.0])
            .with_min_inner_size([400.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Wot Blitz FreeXP Calc.",
        options,
        Box::new(|_| Box::<WotbApp>::default()),
    )
}
