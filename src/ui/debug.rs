use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

#[derive(Debug, Default)]
pub(super) struct DebugPopup<'a> {
    pub title: Line<'a>,
    pub content: Text<'a>,
    pub border_style: Style,
    pub title_style: Style,
    pub style: Style,
}

impl Widget for DebugPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
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

impl<'a> DebugPopup<'a> {
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
        Self {
            title: self.title,
            content: Text::from(content),
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
