use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::{
    clients::float_client,
    widgets::layouts::{LoggingPanel, MenuPanel},
};

mod clients;
mod widgets;

#[derive(PartialEq, Debug, Default)]
enum FocusedWidgetArea {
    #[default]
    Menu,
    Actions,
}

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    focused_widget: FocusedWidgetArea,
    exit: bool,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(30),
                Constraint::Length(2),
                Constraint::Percentage(68),
            ])
            .split(frame.area());

        let menu_panel = MenuPanel {
            title: String::from("Hello"),
            is_focused: self.focused_widget == FocusedWidgetArea::Menu,
        };

        let logging_panel = LoggingPanel {
            title: String::from("World"),
            is_focused: self.focused_widget == FocusedWidgetArea::Actions,
        };

        frame.render_widget(menu_panel, layout[0]);
        frame.render_widget(logging_panel, layout[2]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            KeyCode::Char('L') => self.self_log_time(),
            KeyCode::Tab => self.next_focus(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    fn decrement_counter(&mut self) {
        self.counter -= 1;
    }

    fn next_focus(&mut self) {
        self.focused_widget = match self.focused_widget {
            FocusedWidgetArea::Menu => FocusedWidgetArea::Actions,
            FocusedWidgetArea::Actions => FocusedWidgetArea::Menu,
        }
    }

    fn self_log_time(&self) {
        let client = float_client::FloatClient::new();
        client.authenticate();
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" LazyFloat ".bold());

        let instructions = Line::from(vec![
            " Login ".into(),
            "L".green().bold(),
            " Logtime ".into(),
            "<Space>".green().bold(),
            " Quit ".into(),
            "<Q> ".green().bold(),
        ]);

        let block = Block::bordered()
            .title(title.left_aligned())
            .title_bottom(instructions.left_aligned())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
