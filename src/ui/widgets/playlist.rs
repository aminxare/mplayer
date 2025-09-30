use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, Borders, List, ListDirection, ListState, Padding, StatefulWidget},
};

use crate::audio::song::Song;

pub struct PlayList;
pub struct PlayListState {
    items: Vec<PlayListItem>,
    list_state: ListState,
}

impl PlayListState {
    pub fn new<T: IntoIterator<Item = Song>>(songs: T) -> Self {
        Self::from_iter(songs)
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }

    pub fn select_last(&mut self) {
        self.list_state.select_last();
    }

    pub fn select_first(&mut self) {
        self.list_state.select_first();
    }

    pub fn selected(&self) -> Option<usize> {
        self.list_state.selected()
    }
}

impl FromIterator<Song> for PlayListState {
    fn from_iter<T: IntoIterator<Item = Song>>(iter: T) -> Self {
        let items = iter.into_iter().map(|s| s.into()).collect();
        let list_state = ListState::default();
        Self { items, list_state }
    }
}

pub struct PlayListItem {
    title: String,
    artist: String,
}

impl From<Song> for PlayListItem {
    fn from(value: Song) -> Self {
        Self {
            title: value.title,
            artist: value.artist,
        }
    }
}

impl StatefulWidget for PlayList {
    type State = PlayListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let items = state
            .items
            .iter()
            .map(|i| format!("{:<50} ({})", i.title.clone(), i.artist.clone()))
            .collect::<Vec<String>>();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(" Playlist ")
                    .title_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .padding(Padding::new(1, 1, 1, 1)),
            )
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, &mut state.list_state);
    }
}
