use crate::gui::app::App;

mod data_types;
mod gui;
mod misc_funcs;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 500.0])
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    eframe::run_native(
        "Wot Blitz FreeXP Calc.",
        options,
        Box::new(|_| Box::<App>::default()),
    )
}
