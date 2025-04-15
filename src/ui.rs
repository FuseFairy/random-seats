use crate::fonts::setup_custom_fonts;
use crate::logic::{assign_students_to_tags, read_student, write_seats_to_excel};
use chrono::Local;
use eframe::egui::{
    self, Align, Button, Color32, FontFamily, FontId, Frame, Label, Layout, ProgressBar, RichText,
    ScrollArea, Stroke, TextEdit, TextWrapMode, Vec2,
};
use std::env;

#[derive(PartialEq)]
enum ProcessingState {
    Idle,       // 閒置
    Resetting,  // 重置中
    Processing, // 處理中
}

#[derive(Clone)]
struct LogEntry {
    timestamp: String,
    message: String,
    is_error: bool,
}

pub struct AppState {
    input_path: String,
    max_seats: usize,
    excluded: String,
    status: String,
    progress: f32,
    logs: Vec<LogEntry>,
    processing_state: ProcessingState,
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self::default()
    }
}

impl Default for AppState {
    fn default() -> Self {
        let current_dir = env::current_dir().expect("無法獲取當前工作目錄");
        // 拼接相對路徑，生成絕對路徑
        let input_path = current_dir
            .join("input")
            .join("student_data.xlsx")
            .to_string_lossy()
            .into_owned();

        Self {
            input_path,
            max_seats: 60,
            excluded: "A011,A012,A023,A024,A034,A035,A045,A046".to_string(),
            status: String::new(),
            progress: 0.0,
            logs: Vec::new(),
            processing_state: ProcessingState::Idle,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let font = FontId {
            size: 18.0,
            family: FontFamily::Name("wenkai".into()),
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(
                    RichText::new("學生座位分配工具")
                        .font(font.clone())
                        .strong()
                        .size(24.0),
                );

                ui.add_space(20.0);

                egui::Grid::new("input_grid")
                    .min_col_width(ui.available_width())
                    .show(ui, |ui| {
                        // 第一行：Input
                        ui.horizontal_centered(|ui| {
                            ui.add(
                                Label::new(
                                    RichText::new("Input path：").font(font.clone()).size(18.0),
                                )
                                .halign(Align::RIGHT),
                            );
                            // 設置輸入框最大寬度
                            ui.add(
                                TextEdit::singleline(&mut self.input_path)
                                    .desired_width(ui.max_rect().width())
                                    .font(font.clone()),
                            );
                        });
                        ui.end_row();

                        // 第二行：Max seats
                        let mut max_seats_str = self.max_seats.to_string(); // 將 usize 轉成字串
                        ui.horizontal_centered(|ui| {
                            ui.add(
                                Label::new(
                                    RichText::new("Max seats：").font(font.clone()).size(18.0),
                                )
                                .halign(Align::RIGHT),
                            );
                            if ui
                                .add(
                                    TextEdit::singleline(&mut max_seats_str)
                                        .desired_width(ui.max_rect().width())
                                        .font(font.clone()),
                                )
                                .changed()
                            {
                                if max_seats_str.is_empty() {
                                    self.max_seats = 0;
                                }
                                // 嘗試轉成 usize，如果成功就更新
                                else if let Ok(val) = max_seats_str.parse::<usize>() {
                                    self.max_seats = val;
                                }
                            }
                        });
                        ui.end_row();

                        // 第三行：排除的座位
                        ui.horizontal_centered(|ui| {
                            ui.add(
                                Label::new(
                                    RichText::new("排除座位：").font(font.clone()).size(18.0),
                                )
                                .halign(Align::RIGHT),
                            );
                            ui.add(
                                TextEdit::multiline(&mut self.excluded)
                                    .desired_width(ui.max_rect().width())
                                    .font(font.clone()),
                            );
                        });
                        ui.end_row();
                    });

                ui.add_space(20.0);

                if ui
                    .add(
                        Button::new(RichText::new("Start").font(font.clone()))
                            .min_size(egui::vec2(100.0, 40.0))
                            .corner_radius(5.0),
                    )
                    .clicked()
                    && self.processing_state == ProcessingState::Idle
                {
                    self.processing_state = ProcessingState::Resetting;
                    self.progress = 0.0;
                    self.status = String::new();
                    ctx.request_repaint();
                }

                match self.processing_state {
                    ProcessingState::Resetting => {
                        self.processing_state = ProcessingState::Processing;
                        ctx.request_repaint();
                    }
                    ProcessingState::Processing => {
                        let excluded_tags: Vec<String> = self
                            .excluded
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();

                        self.progress = 0.2;
                        let current_time =
                            Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

                        match read_student(&self.input_path) {
                            Ok(students) => {
                                let student_count = students.len();

                                if self.max_seats < student_count {
                                    self.status = format!(
                                        "座位數 {} 小於學生數量 {}！",
                                        self.max_seats, student_count
                                    );
                                    self.logs.push(LogEntry {
                                        timestamp: current_time.unwrap(),
                                        message: self.status.clone(),
                                        is_error: true,
                                    });
                                    self.progress = 0.0;
                                } else {
                                    self.progress = 0.3;
                                    match assign_students_to_tags(
                                        &students,
                                        self.max_seats,
                                        &excluded_tags,
                                    )
                                    .and_then(|assignments| {
                                        self.progress = 0.6;
                                        write_seats_to_excel(&self.input_path, &assignments)?;
                                        Ok(())
                                    }) {
                                        Ok(_) => {
                                            self.progress = 1.0;
                                            self.status = "座位已成功寫入！".to_string();
                                            self.logs.push(LogEntry {
                                                timestamp: current_time.unwrap(),
                                                message: self.status.clone(),
                                                is_error: false,
                                            });
                                        }
                                        Err(e) => {
                                            self.status = format!("{}", e);
                                            self.logs.push(LogEntry {
                                                timestamp: current_time.unwrap(),
                                                message: self.status.clone(),
                                                is_error: true,
                                            });
                                            self.progress = 0.0;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                self.status = format!("{}", e);
                                self.logs.push(LogEntry {
                                    timestamp: current_time.unwrap(),
                                    message: self.status.clone(),
                                    is_error: true,
                                });
                                self.progress = 0.0;
                            }
                        }

                        self.processing_state = ProcessingState::Idle;
                    }
                    ProcessingState::Idle => {}
                }
                ui.add_space(50.0);

                ui.add(ProgressBar::new(self.progress).desired_width(ui.available_width()));

                let percentage = (self.progress * 100.0).round();
                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.label(RichText::new(format!("{}%", percentage)).font(font.clone()));
                });

                ui.add_space(30.0);

                let available_height = ui.available_height(); // 剩餘空間高度

                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), available_height),
                    Layout::top_down_justified(Align::Min),
                    |ui| {
                        Frame::group(ui.style())
                            .fill(Color32::from_rgba_premultiplied(64, 64, 64, 250)) // 背景顏色
                            .corner_radius(5.0)
                            .stroke(Stroke::new(1.0, Color32::DARK_GRAY)) // 邊框
                            .show(ui, |ui| {
                                ScrollArea::both()
                                    .auto_shrink([false, false])
                                    .stick_to_bottom(true) // 捲軸自動貼底部
                                    .show(ui, |ui| {
                                        for log in &self.logs {
                                            let color = if log.is_error {
                                                Color32::LIGHT_RED
                                            } else {
                                                Color32::LIGHT_GREEN
                                            };
                                            ui.add(
                                                Label::new(
                                                    RichText::new(format!(
                                                        "{} -> {}",
                                                        log.timestamp, log.message
                                                    ))
                                                    .font(font.clone())
                                                    .size(16.0)
                                                    .color(color),
                                                )
                                                .wrap_mode(TextWrapMode::Extend),
                                            );
                                        }
                                    });
                            });
                    },
                );
            });
        });
    }
}
