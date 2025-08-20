use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::ui::components;

pub fn views(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(15), // top ---> headers, tabs and ...
            Constraint::Percentage(60),    // main
            Constraint::Percentage(25), // footer
        ],
    )
    .split(frame.area());

    let block1 = Block::default()
        .title(" MPlayer ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let block2 = Block::default()
        .title(" MPlayer ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let text1 = Paragraph::new("HEADER").block(block1);
    let text2 = Paragraph::new("MAIN").block(block2);

    frame.render_widget(text1, main_layout[0]);
    frame.render_widget(text2, main_layout[1]);
    components::now_playing::draw_music_player(frame, main_layout[2]);
}
