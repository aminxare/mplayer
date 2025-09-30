use std::{cell::RefCell, rc::Rc};

use crate::audio::{player::AudioPlayer, song::Song};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::ui::widgets::{
    playlist::{PlayList, PlayListState},
    status_bar::{StatusBar, StatusbarState},
};

mod widgets;

pub struct UI {
    pub list_state: PlayListState,
    pub status_message: Rc<RefCell<String>>,
}

impl UI {
    pub fn new(audio_player: &AudioPlayer, status_message: Rc<RefCell<String>>) -> Self {
        let list_state = PlayListState::new(audio_player.get_songs().to_vec());

        Self {
            list_state,
            status_message,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, current_song: Option<Song>) {
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Min(1),         // main
                Constraint::Percentage(20), // footer
                Constraint::Length(2),      // statusbar
            ],
        )
        .spacing(0)
        .split(frame.area());

        let play_list = PlayList;

        frame.render_stateful_widget(play_list, main_layout[0], &mut self.list_state);

        frame.render_widget(
            &widgets::music_player::MusicPlayer { song: current_song },
            main_layout[1],
        );
        frame.render_stateful_widget(
            StatusBar,
            main_layout[2],
            &mut StatusbarState {
                message: self.status_message.borrow().clone(),
            },
        );
    }
}
