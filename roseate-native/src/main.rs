#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::{env, path::Path};

use image::Image;
use eframe::egui::{self, ImageSource};

mod image;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let cli_args: Vec<String> = env::args().collect();
    let image_path = cli_args.get(1);

    let image = match image_path {
        Some(path) => {
            let path = Path::new(path);
            Some(Image::from_path(path))
        },
        None => None
    };

    eframe::run_native(
        "Roseate",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Roseate::new(image)))
        }),
    )
}

#[derive(Default)]
struct Roseate {
    image: Option<Image>
}

impl Roseate {
    fn new(image: Option<Image>) -> Self {
        Self { image }
    }
}

impl eframe::App for Roseate {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            let window_rect = ctx.input(|i: &egui::InputState| i.screen_rect());

            if !self.image.is_none() {
                let image = self.image.as_ref().unwrap();

                ui.centered_and_justified(|ui| {
                    egui::ScrollArea::both().show(ui, |ui| {
                        let scale_x = window_rect.width() / image.image_size.width as f32;
                        let scale_y = window_rect.height() / image.image_size.height as f32;
    
                        let scale_factor = scale_x.min(scale_y); // Scale uniformly.
    
                        // Make sure scale_factor doesn't exceed the original size (1).
                        let scale_factor = scale_factor.min(1.0);
    
                        let scaled_image_width = image.image_size.width as f32 * scale_factor;
                        let scaled_image_height = image.image_size.height as f32 * scale_factor;

                        let scaled_image_width_animated = egui_animation::animate_eased(
                            ctx, "image_scale_width", scaled_image_width, 1.5, simple_easing::cubic_in_out
                        ) as u32;
                        let scaled_image_height_animated = egui_animation::animate_eased(
                            ctx, "image_scale_height", scaled_image_height, 1.5, simple_easing::cubic_in_out
                        ) as u32;

                        ui.add(
                            egui::Image::from_bytes(
                                format!("bytes://{}", image.image_path), image.image_bytes.clone()
                            ).max_width(scaled_image_width_animated as f32).max_height(scaled_image_height_animated as f32).rounding(10.0)
                        );
                    });
                });

            } else {
                ui.centered_and_justified(|ui| {
                    ui.add(egui::Image::new(get_platform_rose_image()).max_width(130.0));
                });
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