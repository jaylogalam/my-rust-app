use color_eyre::eyre::Result;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Buffer, Rect};
use ratatui::widgets::{Block, Borders, Widget};
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
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(area);

        Paragraph::new("Explorer")
            .bold()
            .centered()
            .block(Block::new().borders(Borders::ALL))
            .render(layout[0], buf);

        Paragraph::new("Content")
            .bold()
            .centered()
            .block(Block::new().borders(Borders::ALL))
            .render(layout[1], buf);

        Paragraph::new("Outline")
            .bold()
            .centered()
            .block(Block::new().borders(Borders::ALL))
            .render(layout[2], buf);
    }
}
