use ratatui::widgets::{Paragraph, Widget};

pub struct StatusBar;

impl Widget for StatusBar {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let t = Paragraph::new("[MODE]");

        t.render(area, buf);
    }
}
