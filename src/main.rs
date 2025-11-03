mod app;
mod ui;
use crate::app::App;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
