use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Visualizer {
    pub is_playing: bool,
}

impl Widget for Visualizer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(" Visualizer ");
        let inner_area = block.inner(area);
        block.render(area, buf);

        if inner_area.width == 0 || inner_area.height == 0 {
            return;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let num_bars = inner_area.width;
        let max_height = inner_area.height;

        for i in 0..num_bars {
            let height = if self.is_playing {
                // Create a dynamic pseudo-random wave effect based on time and bar index
                let sine = ((now as f64 / 150.0) + (i as f64 * 0.5)).sin();
                let cosine = ((now as f64 / 250.0) + (i as f64 * 0.8)).cos();
                let combined = (sine + cosine + 2.0) / 4.0; // Normalized 0.0 to 1.0
                (combined * max_height as f64).round() as u16
            } else {
                0
            };

            for y in 0..height {
                if y < max_height {
                    let symbol = if y == height - 1 && height > 0 { "▄" } else { "█" };
                    let color = match y {
                        y if y > max_height * 2 / 3 => Color::Red,
                        y if y > max_height / 3 => Color::Yellow,
                        _ => Color::Green,
                    };

                    buf.get_mut(inner_area.x + i, inner_area.y + inner_area.height - 1 - y)
                        .set_symbol(symbol)
                        .set_style(Style::default().fg(color));
                }
            }
        }
    }
}
