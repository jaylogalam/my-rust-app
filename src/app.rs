use color_eyre::eyre::Result;
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame, style::Stylize, widgets::Paragraph};

pub struct App {
    pub(crate) exit: bool,
}

impl App {
    pub fn init() -> App {
        App { exit: false }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            crate::keybinds::handle_key_event(self)?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
