use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};
use std::time::{SystemTime, UNIX_EPOCH};

const BRAILLE_BASE: u16 = 0x2800;

const BRAILLE_DOTS: [[u16; 4]; 2] = [
    [0x01, 0x08, 0x40, 0x02],
    [0x10, 0x20, 0x04, 0x80],
];

pub struct Visualizer {
    pub is_playing: bool,
}

impl Widget for Visualizer {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(" Visualizer ");
        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width == 0 || inner.height == 0 {
            return;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let cols = inner.width as usize;
        let rows = inner.height as usize;
        let height_pixels = rows * 4;
        let mid = height_pixels as f64 / 2.0;

        for col in 0..cols {
            let mut grid = [[false; 4]; 2];

            for (dx, col_dots) in grid.iter_mut().enumerate() {
                let x_norm = ((col * 2 + dx) as f64) / ((cols * 2) as f64);

                let y = if self.is_playing {
                    let wave1 = (x_norm * 8.0 + now as f64 / 400.0).sin() * 0.35;
                    let wave2 = (x_norm * 12.0 - now as f64 / 600.0).sin() * 0.2;
                    let wave3 = (x_norm * 4.0 + now as f64 / 1000.0).sin() * 0.15;
                    let envelope = (x_norm * std::f64::consts::PI).sin();
                    (wave1 + wave2 + wave3) * envelope
                } else {
                    let phase = now as f64 / 2000.0;
                    (x_norm * 4.0 + phase).sin() * 0.03
                };

                let pixel_y = mid - y * mid;
                let pixel_y = pixel_y.clamp(0.0, (height_pixels - 1) as f64) as usize;
                let row = pixel_y / 4;
                let bit = pixel_y % 4;
                if row < rows {
                    col_dots[bit] = true;
                }
            }

            for row in 0..rows {
                let mut braille_char = BRAILLE_BASE;
                for (dx, col_dots) in grid.iter().enumerate() {
                    for (bit, &set) in col_dots.iter().enumerate() {
                        if set {
                            braille_char += BRAILLE_DOTS[dx][bit];
                        }
                    }
                }

                let symbol = char::from_u32(braille_char as u32).unwrap();
                let cell = &mut buf[(inner.x + col as u16, inner.y + row as u16)];

                let dist_from_mid = if row < rows / 2 {
                    (rows / 2 - row) as f64 / (rows as f64)
                } else {
                    (row - rows / 2) as f64 / (rows as f64)
                };

                let color = if self.is_playing {
                    let r = (100.0 - dist_from_mid * 80.0) as u8;
                    let g = (220.0 - dist_from_mid * 100.0) as u8;
                    let b = 255;
                    Color::Rgb(r, g, b)
                } else {
                    let v = (80.0 - dist_from_mid * 40.0) as u8;
                    Color::Rgb(v, v, v + 20)
                };

                cell.set_symbol(&symbol.to_string())
                    .set_style(Style::default().fg(color));
            }
        }
    }
}
