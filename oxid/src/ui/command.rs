use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

#[derive(Debug, Default)]
pub(super) struct CommandPopup<'a> {
    pub title: Line<'a>,
    pub content: Text<'a>,
    pub border_style: Style,
    pub title_style: Style,
    pub style: Style,
}

impl Widget for CommandPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = Block::new()
            .bg(Color::Rgb(40, 30, 51))
            .title(self.title)
            .title_alignment(Alignment::Center)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}

impl<'a> CommandPopup<'a> {
    pub fn title(self, title: &'a str) -> Self {
        Self {
            title: Line::from(title),
            content: self.content,
            border_style: self.border_style,
            title_style: self.title_style,
            style: self.style,
        }
    }
    pub fn content(self, content: &'a str) -> Self {
        let mut prefix = String::from(":");
        prefix.push_str(content);

        Self {
            title: self.title,
            content: Text::from(prefix),
            border_style: self.border_style,
            title_style: self.title_style,
            style: self.style,
        }
    }

    pub fn style(self, style: Style) -> Self {
        Self {
            title: self.title,
            content: self.content,
            border_style: self.border_style,
            title_style: self.title_style,
            style,
        }
    }

    pub fn title_style(self, title_style: Style) -> Self {
        Self {
            title: self.title,
            content: self.content,
            border_style: self.border_style,
            style: self.style,
            title_style,
        }
    }

    pub fn border_style(self, border_style: Style) -> Self {
        Self {
            title: self.title,
            content: self.content,
            style: self.style,
            title_style: self.title_style,
            border_style,
        }
    }
}
