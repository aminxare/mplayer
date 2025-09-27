use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    app::state::AppState,
    ui::widgets::{
        self,
        playlist::PlayList,
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
    // let text2 = Paragraph::new("MAIN").block(block2);

    let play_list = PlayList;

    frame.render_widget(text1, main_layout[0]);
    frame.render_widget(play_list, main_layout[1]);
    frame.render_stateful_widget(
        StatusBar,
        main_layout[3],
        &mut StatusbarState {
            message: state.status_message.clone(),
        },
    );

    let song = state.audio_player.current_song();

    frame.render_widget(&widgets::music_player::MusicPlayer { song }, main_layout[2]);
}
