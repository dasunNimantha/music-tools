use music_tools::MusicToolsApp;
use iced::{Application, Settings, Pixels, Font};
use music_tools::config::window;

fn main() -> iced::Result {
    let fira_sans_font = Font::with_name("Fira Sans");
    
    MusicToolsApp::run(Settings {
        window: iced::window::Settings {
            size: window::default_size(),
            min_size: Some(window::min_size()),
            ..Default::default()
        },
        fonts: vec![iced_aw::BOOTSTRAP_FONT_BYTES.into()],
        default_font: fira_sans_font,
        default_text_size: Pixels(14.0),
        ..Default::default()
    })
}
