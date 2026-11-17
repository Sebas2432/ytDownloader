use iced::widget::{button, column, text, text_input, Column};
use ytdown::Ytdown;

#[derive(Debug, Clone)]
enum Message {
    LinkChanged(String),
    DownloadPressed,
    DownloadResult(Result<(), String>)
}

#[derive(Debug, Default)]
struct Link {
    y_link: String,
}

async fn download_file(url: &str) -> Result<(), ytdown::Error> {
    let yt = Ytdown::builder().build()?;
    let info = yt.resolve(url).await?;
        if let ytdown::MediaInfo::Single(video) = info {
        let fmt = video.formats().best_progressive()?;
        yt.download(fmt, std::path::Path::new("out.mp4"))
            .progress(|p| {
                if let Some(pct) = p.percent() {
                    eprintln!("{pct:.1}%");
                }
            })
            .await?;
    }
    Ok(())
}

impl Link{
    fn view(&self) -> Column<'_, Message> {
        column![
            text("Enter YouTube link:"),
            text_input("Enter YouTube link...", &self.y_link).on_input(Message::LinkChanged),
            button("Download").on_press(Message::DownloadPressed),
        ]
    }
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::LinkChanged(url) => {
                self.y_link = url;
                iced::Task::none()
            },
             Message::DownloadPressed => {
                let raw = self.y_link.trim().to_string();
                let url = if raw.starts_with("http") { raw } else { format!("https://{raw}") };
                iced::Task::perform(async move {
                    download_file(&url)
                        .await
                        .map_err(|e| e.to_string())
                }, Message::DownloadResult)
            },
            Message::DownloadResult(_result) => {
                match _result {
                    Ok(())  => println!("[dbg] OK -> out.mp4"),
                    Err(e)  => eprintln!("[dbg] FAILED: {e}"),
                }
                iced::Task::none()
            }
        }
    }
}

fn main() {
    let _ = iced::run(Link::update, Link::view);
}