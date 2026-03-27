use color_eyre::eyre::Result;
use ratatui::{DefaultTerminal, Frame, style::Stylize, widgets::Paragraph};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app = App { exit: false };

    let run = app.run(&mut terminal);

    ratatui::restore();
    run
}

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::Widget;

pub struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_key_event()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self) -> Result<()> {
        if let event::Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char(' ') => self.handle_space_key_event()?,
                event::KeyCode::Char('q') => {}
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_space_key_event(&mut self) -> Result<()> {
        if let event::Event::Key(next_key) = event::read()? {
            match next_key.code {
                event::KeyCode::Char('x') => { /* do something after Space→x */ }
                event::KeyCode::Char('q') => self.exit = true,
                _ => {}
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("Hello World").bold().render(area, buf);
    }
}
