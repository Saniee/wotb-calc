use gui::MyApp;

mod gui;
mod tank_data_types;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Wot Blitz Tool",
        options,
        Box::new(|_| Box::<MyApp>::default()),
    )
}
