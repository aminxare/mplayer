use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn views(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3), // top ---> headers, tabs and ...
            Constraint::Min(0),    // main
            Constraint::Length(3), // footer
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

    let block3 = Block::default()
        .title(" MPlayer ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let text1 = Paragraph::new("HEADER").block(block1);
    let text2 = Paragraph::new("MAIN").block(block2);
    let text3 = Paragraph::new("FOOTER").block(block3);

    frame.render_widget(text1, main_layout[0]);
    frame.render_widget(text2, main_layout[1]);
    frame.render_widget(text3, main_layout[2]);
}
