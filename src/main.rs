#![windows_subsystem = "windows"] // 隱藏控制台窗口，只對 windows 系統有效

mod fonts;
mod logic;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    eframe::run_native(
        "學生座位分配工具",
        options,
        Box::new(|cc| Ok(Box::new(ui::AppState::new(cc)))),
    )
}
