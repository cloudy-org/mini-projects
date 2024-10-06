#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{env, path::Path, sync::RwLock};

use image::Image;
use eframe::egui::{self, ImageSource};

mod image;

static IMAGE: RwLock<Option<Image>> = RwLock::new(None);

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let cli_args: Vec<String> = env::args().collect();
    let image_path = cli_args.get(1);

    if image_path != None {
        let path = Path::new(image_path.unwrap());

        let mut image = IMAGE.write().unwrap();
        *image = Some(Image::from_path(path));
    }

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
struct Roseate {}

impl eframe::App for Roseate {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            let window_rect = ctx.input(|i: &egui::InputState| i.screen_rect());

            let image_lock = IMAGE.read().unwrap();

            match image_lock.as_ref() {
                Some(image) => {
                    ui.centered_and_justified(|ui| {
                        egui::ScrollArea::both().show(ui, |ui| {
                            ui.add(
                                egui::Image::new(image.image_source.clone())
                                    .fit_to_original_size(1.0)
                                    .rounding(20.0)
                            );
                        });
                    });
                },
                None => {
                    ui.centered_and_justified(|ui| {
                        ui.add(egui::Image::new(get_platform_rose_image()).max_width(150.0));
                    });
                }
            }
        });

    }

}

fn get_platform_rose_image<'a>() -> ImageSource<'a> {
    if cfg!(target_os = "windows") {
        return egui::include_image!("../assets/rose_emojis/microsoft.png");
    } else if cfg!(target_os = "macos") {
        return egui::include_image!("../assets/rose_emojis/apple.png");
    }

    return egui::include_image!("../assets/rose_emojis/google_noto.png");
}