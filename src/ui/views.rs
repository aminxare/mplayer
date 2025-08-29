use std::{cell::RefCell, path::PathBuf, rc::Rc};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    app::state::AppState,
    audio::song::Song,
    ui::widgets::{
        self,
        status_bar::{StatusBar, StatusbarState},
    },
};

pub fn views(frame: &mut Frame, state: &AppState) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(15), // top ---> headers, tabs and ...
            Constraint::Percentage(60), // main
            Constraint::Percentage(20), // footer
            Constraint::Length(2),      // statusbar
        ],
    )
    .spacing(0)
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
    frame.render_stateful_widget(
        StatusBar,
        main_layout[3],
        &mut StatusbarState {
            message: state.status_message.clone(),
        },
    );

    let song = Rc::new(RefCell::new(Song {
        artist: "a".into(),
        duration: 100,
        progress: 20,
        title: "t".into(),
        path: PathBuf::new(),
    }));

    frame.render_widget(&widgets::music_player::MusicPlayer { song }, main_layout[2]);
}
