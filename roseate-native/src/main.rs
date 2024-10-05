#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use egui_twemoji::EmojiLabel;
use image::Image;
use egui::load::Bytes;
use eframe::egui::{self, vec2, FontId};

mod image;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Roseate",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<Roseate>::default())
        }),
    )
}

#[derive(Default)]
struct Roseate<'a> {
    image: Option<Image<'a>>
}

impl eframe::App for Roseate<'_> {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {

            egui::ScrollArea::both().show(ui, |ui| {

                match &self.image {
                    Some(image) => {
                        ui.add(
                            egui::Image::new(image.image_source.clone())
                                .fit_to_original_size(1.0)
                                .rounding(20.0)
                        );
                    },
                    None => {
                        EmojiLabel::new("🌹").show(ui);
                    }
                }

            });

        });

    }

}