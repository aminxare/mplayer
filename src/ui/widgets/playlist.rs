use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, List, ListDirection, ListItem, ListState, StatefulWidget, Widget},
};

pub struct PlayList;

impl Widget for PlayList {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let items = [
            ListItem::new(Text::from("boo").right_aligned()),
            ListItem::new(Text::from("foo").right_aligned()),
        ];
        let list = List::new(items)
            .block(Block::bordered().title("List"))
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
        
        Widget::render(list, area, buf);
    }
}
