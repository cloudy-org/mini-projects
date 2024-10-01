use iced::widget::image::Handle;
use iced::widget::{self, text};
use iced::{Element, Font};
use std::path::PathBuf;
use rfd::FileDialog;

#[derive(Default)]
struct Roseate {
    image: Option<Handle>,
    load_button_pressed: bool,
}

#[derive(Debug, Clone)]
enum Message {
    LoadImage,
    ImageLoaded(Option<Handle>),
}

impl Roseate {
    fn update(&mut self, message: Message) {
        match message {
            Message::LoadImage => {
                let path = FileDialog::new().add_filter("Image", &["png", "jpg", "jpeg"]).pick_file();

                if let Some(path) = path {
                    let handle = Handle::from_path(path);
                    self.image = Some(handle);
                }
            }
            Message::ImageLoaded(image) => {
                self.image = image;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let load_button = widget::Button::new(
            widget::Text::new("🌹").size(25).shaping(text::Shaping::Advanced)
        ).on_press(Message::LoadImage);

        // Image::new(image.clone()).width(Length::Units(500)).height(Length::Units(500));

        widget::Container::new(load_button).into()
    }
}

fn main() -> iced::Result {
    iced::application("Roseate", Roseate::update, Roseate::view)
        .run()
}