use ratatui::{
    prelude::*,
    text::{Line, Text},
    widgets::{Paragraph, Wrap},
};

use crate::app::{AppState, FilterType, InputMode};

pub fn render(state: &AppState) -> Paragraph {
    let mut lines = vec![
        Line::from(format!("Start 1 [1]: {}", state.start1)),
        Line::from(format!("Start 2 [2]: {}", state.start2)),
        Line::from(format!("Range Start [s]: {}", state.range_start)),
        Line::from(format!("Range End [e]: {}", state.range_end)),
        Line::from(format!("Filter Value [v]: {}", state.filter_value)),
        Line::from(""),
        Line::from("Filters:"),
    ];

    for filter in &state.filters {
        let symbol = match filter.filter_type {
            FilterType::Ge => "≥",
            FilterType::Le => "≤",
        };
        lines.push(Line::from(format!("{} {}", symbol, filter.value)));
    }

    lines.extend([
        Line::from(""),
        Line::from("[a]dd filters  [d]elete filter"),
        Line::from("[r]ecalculate  [c]lear filters"),
        Line::from("Arrow keys: Navigate results"),
        Line::from("Press 1,2,s,e,v to edit fields"),
    ]);

    // Highlight active field
    if state.input_mode != InputMode::Normal {
        let field_index = match state.input_mode {
            InputMode::Start1 => 0,
            InputMode::Start2 => 1,
            InputMode::RangeStart => 2,
            InputMode::RangeEnd => 3,
            InputMode::FilterValue => 4,
            _ => 0,
        };

        if let Some(line) = lines.get_mut(field_index) {
            *line = line.clone().style(Style::new().yellow());
        }
    }

    // Show error if exists
    let mut text = Text::from(lines);
    if let Some(err) = &state.error {
        text.lines
            .push(Line::from(err.clone()).style(Style::new().red()));
    }

    Paragraph::new(text).wrap(Wrap { trim: true })
}
