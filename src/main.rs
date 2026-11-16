use iced::widget::{button, column, text, text_input, Column};
use std::fs::File;
use std::io::copy;
use reqwest;

#[derive(Debug, Clone)]
enum Message {
    LinkChanged(String),
    DownloadPressed,
}

#[derive(Debug, Default)]
struct Link {
    y_link: String
}

fn download_file(url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::blocking::get(url)?;
    let mut dest = File::create(destination)?;
    copy(&mut response, &mut dest)?;
    Ok(())
}

impl Link{
    fn view(&self) -> Column<'_, Message> {
        column![
            text("Enter YouTube link:"),
            text_input("Enter YouTube link...", &self.y_link).on_input(Message::LinkChanged),
            button("Download").on_press(Message::DownloadPressed)
        ]
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::DownloadPressed => {
                println!("Download button pressed for link: {}", self.y_link);
                if self.y_link.is_empty() {
                    println!("No link provided.");
                    return;
                }
                download_file(&self.y_link, "output.mp4").expect("Failed to download file");
            }
            Message::LinkChanged(link) => {
                self.y_link = link;
            }
        }
    }
}

fn main() {
    let _ = iced::run(Link::update, Link::view);
}