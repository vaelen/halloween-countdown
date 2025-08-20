// Halloween Countdown Application
// Copyright (c) 2025 Andrew C. Young <andrew@vaelen.org>
// Licensed under the MIT License - see LICENSE file for details

// For Laine <3

use eframe::egui;
use chrono::{Datelike, Local, NaiveDate};
use std::time::{Duration, Instant};

const HENNY_PENNY_FONT: &[u8] = include_bytes!("../assets/HennyPenny-Regular.ttf");
const MYSTERY_QUEST_FONT: &[u8] = include_bytes!("../assets/MysteryQuest-Regular.ttf");

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([450.0, 250.0])
            .with_min_inner_size([450.0, 250.0])
            .with_max_inner_size([450.0, 250.0])
            .with_resizable(false)
            .with_title("Halloween Countdown"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Halloween Countdown",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(HalloweenApp::default()))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    fonts.font_data.insert(
        "henny_penny".to_owned(),
        egui::FontData::from_static(HENNY_PENNY_FONT),
    );
    
    fonts.font_data.insert(
        "mystery_quest".to_owned(),
        egui::FontData::from_static(MYSTERY_QUEST_FONT),
    );
    
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "henny_penny".to_owned());
    
    fonts
        .families
        .entry(egui::FontFamily::Name("mystery_quest".into()))
        .or_default()
        .insert(0, "mystery_quest".to_owned());
    
    ctx.set_fonts(fonts);
}

struct HalloweenApp {
    last_update: Instant,
}

impl Default for HalloweenApp {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
        }
    }
}

impl HalloweenApp {
    fn days_until_halloween(&self) -> i64 {
        let today = Local::now().date_naive();
        let current_year = today.year();
        
        let mut halloween = NaiveDate::from_ymd_opt(current_year, 10, 31).unwrap();
        
        if today > halloween {
            halloween = NaiveDate::from_ymd_opt(current_year + 1, 10, 31).unwrap();
        }
        
        (halloween - today).num_days()
    }
}

impl eframe::App for HalloweenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check if 60 seconds have passed since last update
        if self.last_update.elapsed() >= Duration::from_secs(60) {
            self.last_update = Instant::now();
            ctx.request_repaint(); // Force a repaint to update the display
        }
        
        let days = self.days_until_halloween();
        
        let bg_color = egui::Color32::from_rgb(26, 0, 51);
        let orange = egui::Color32::from_rgb(255, 102, 0);
        let green = egui::Color32::from_rgb(0, 255, 0);
        
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    0.0,
                    bg_color,
                );
                
                ui.with_layout(
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        ui.add_space(50.0);
                        
                        if days == 0 {
                            // Special Halloween day display
                            ui.label(
                                egui::RichText::new("TODAY")
                                    .size(72.0)
                                    .color(orange)
                                    .strong()
                                    .family(egui::FontFamily::Name("mystery_quest".into()))
                            );
                            
                            ui.add_space(10.0);
                            
                            ui.label(
                                egui::RichText::new("is Halloween!")
                                    .size(28.0)
                                    .color(green)
                            );
                        } else {
                            // Regular countdown display
                            let days_text = if days == 1 {
                                "1 DAY".to_string()
                            } else {
                                format!("{} DAYS", days)
                            };
                            
                            ui.label(
                                egui::RichText::new(days_text)
                                    .size(72.0)
                                    .color(orange)
                                    .strong()
                                    .family(egui::FontFamily::Name("mystery_quest".into()))
                            );
                            
                            ui.add_space(10.0);
                            
                            ui.label(
                                egui::RichText::new("Until Halloween")
                                    .size(28.0)
                                    .color(green)
                            );
                        }
                    },
                );
            });
        
        // Request repaint continuously every second to ensure timer checking
        // This ensures the app updates even when in background
        ctx.request_repaint_after(Duration::from_secs(1));
    }
}
