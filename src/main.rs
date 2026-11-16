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
    yLink: String
}

fn download_file(url: &str, destination: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::blocking::get(url)?;
    let mut dest = File::create(destination)?;
    copy(&mut response, &mut dest)?;
    Ok(())
}

impl Link{
    fn view(&self) -> Column<Message> {
        column![
            text("Enter YouTube link:"),
            text_input("Enter YouTube link...", &self.yLink).on_input(Message::LinkChanged),
            button("Download").on_press(Message::DownloadPressed)
        ]
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::DownloadPressed => {
                println!("Download button pressed for link: {}", self.yLink);
                if self.yLink.is_empty() {
                    println!("No link provided.");
                    return;
                }
                download_file(&self.yLink, "output.mp4").expect("Failed to download file");
            }
            Message::LinkChanged(link) => {
                self.yLink = link;
            }
        }
    }
}

fn main() {
    iced::run(Link::update, Link::view);
}