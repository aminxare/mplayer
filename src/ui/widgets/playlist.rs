use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, List, ListDirection, ListState, Padding, StatefulWidget},
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
    // TODO: implement another fields
}

impl From<Song> for PlayListItem {
    fn from(value: Song) -> Self {
        Self { title: value.title }
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
            .map(|i| i.title.clone())
            .collect::<Vec<String>>();

        let list = List::new(items)
            .block(
                Block::bordered()
                    .title("List")
                    .border_type(ratatui::widgets::BorderType::Rounded)
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
