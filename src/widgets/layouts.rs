use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Widget},
};

#[derive(Default)]
pub struct MenuPanel {
    pub title: String,
    pub is_focused: bool,
}

impl Widget for MenuPanel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let label = Span::styled("Menu", Modifier::BOLD).yellow();
        let block = Block::default()
            .title_bottom(label)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .padding(Padding::symmetric(2, 1));

        let renderable_block = if self.is_focused {
            block.yellow()
        } else {
            block
        };

        let inner = renderable_block.inner(area);
        renderable_block.render(area, buf);

        let items = vec![
            ListItem::new("Login"),
            ListItem::new("View Projects"),
            ListItem::new("Settings"),
            ListItem::new("Exit"),
        ];

        let list = List::new(items);
        list.render(inner, buf);
    }
}

#[derive(Default)]
pub struct LoggingPanel {
    pub title: String,
    pub is_focused: bool,
}

impl Widget for LoggingPanel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let label = Span::styled("Actions", Modifier::BOLD).red();
        let block = Block::default()
            .title_bottom(label)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        let renderable_block = if self.is_focused {
            block.yellow()
        } else {
            block
        };

        let inner = renderable_block.inner(area);
        renderable_block.render(area, buf);

        let content = Paragraph::new("This is some test test for my layout").centered();
        content.render(inner, buf);
    }
}
