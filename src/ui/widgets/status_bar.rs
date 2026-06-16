use ratatui::{
    style::{Color, Style},
    widgets::{Paragraph, StatefulWidget, Widget},
};

pub struct StatusbarState {
    pub message: String,
}

pub struct StatusBar;
impl StatefulWidget for StatusBar {
    type State = StatusbarState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) where
        Self: Sized,
    {
        let t = Paragraph::new(format!(" Status: {}", state.message))
            .style(Style::default().fg(Color::Gray));
        t.render(area, buf);
    }
}
