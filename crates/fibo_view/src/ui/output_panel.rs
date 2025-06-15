use crate::app::AppState;
use ratatui::widgets::Block;
use ratatui::{
    prelude::*,
    widgets::{List, ListItem},
};

pub fn render(state: &mut AppState) -> List {
    let items: Vec<_> = state
        .results
        .iter()
        .enumerate()
        .map(|(i, num)| ListItem::new(format!("{:4}: {}", i, num)))
        .collect();

    List::new(items)
        .highlight_style(Style::new().yellow())
        .block(Block::bordered())
}
