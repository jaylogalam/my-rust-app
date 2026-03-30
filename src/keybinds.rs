use crate::app::App;
use color_eyre::eyre::Result;
use crossterm::event;

pub fn handle_key_event(app: &mut App) -> Result<()> {
    if let event::Event::Key(key) = event::read()? {
        match key.code {
            event::KeyCode::Char(' ') => handle_space_key_event(app)?,
            event::KeyCode::Char('q') => {}
            _ => {}
        }
    }
    Ok(())
}

fn handle_space_key_event(app: &mut App) -> Result<()> {
    if let event::Event::Key(next_key) = event::read()? {
        match next_key.code {
            event::KeyCode::Char('x') => { /* do something after Space→x */ }
            event::KeyCode::Char('q') => app.exit = true,
            _ => {}
        }
    }
    Ok(())
}
