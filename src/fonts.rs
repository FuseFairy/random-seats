use eframe::egui;
use eframe::egui::{FontData, FontDefinitions, FontFamily};
use std::collections::BTreeMap;
use std::sync::Arc;

pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default(); // 使用 default 保留系統字體

    // 載入字體
    fonts.font_data.insert(
        "wenkai".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/LXGWWenKaiMonoTC-Regular.ttf"
        ))),
    );

    // 創建自定義字體家族
    let mut newfam = BTreeMap::new();
    newfam.insert(FontFamily::Name("wenkai".into()), vec!["wenkai".to_owned()]);
    fonts.families.append(&mut newfam);

    // 設定字體
    ctx.set_fonts(fonts);
}
