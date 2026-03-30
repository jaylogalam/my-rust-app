use color_eyre::eyre::Result;

mod app;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app = App::init();

    let run = app.run(&mut terminal);

    ratatui::restore();

    run
}
