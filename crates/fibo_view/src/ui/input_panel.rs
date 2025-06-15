use crate::app::{AppState, InputMode};
use ratatui::{
    prelude::*,
    text::{Line, Text},
    widgets::{Paragraph, Wrap},
};

pub fn render(state: &AppState) -> Paragraph {
    let mut lines = vec![
        Line::from(""),
        Line::from(format!("🔢 Start Number 1 [1]: {}", state.start1)).style(
            if state.input_mode == InputMode::Start1 {
                Style::new().bold().yellow()
            } else {
                Style::new().white()
            },
        ),
        Line::from(format!("🔢 Start Number 2 [2]: {}", state.start2)).style(
            if state.input_mode == InputMode::Start2 {
                Style::new().bold().yellow()
            } else {
                Style::new().white()
            },
        ),
        Line::from(""),
        Line::from(format!("📍 Range Start [s]: {}", state.range_start)).style(
            if state.input_mode == InputMode::RangeStart {
                Style::new().bold().yellow()
            } else {
                Style::new().light_blue()
            },
        ),
        Line::from(format!("📍 Range End [e]: {}", state.range_end)).style(
            if state.input_mode == InputMode::RangeEnd {
                Style::new().bold().yellow()
            } else {
                Style::new().light_blue()
            },
        ),
        Line::from(""),
        Line::from(format!(
            "🔍 Filter Value [v]: {}{}",
            state.filter_type, state.filter_value
        ))
        .style(if state.input_mode == InputMode::FilterValue {
            Style::new().bold().yellow()
        } else {
            Style::new().light_green()
        }),
        Line::from(""),
        Line::from("🔍 Active Filters:").style(Style::new().bold().magenta()),
    ];

    if state.filters.is_empty() {
        lines.push(Line::from("   (No filters applied)").style(Style::new().italic().dark_gray()));
    } else {
        for (i, filter) in state.filters.iter().enumerate() {
            lines.push(
                Line::from(format!(
                    "   {}. {} {}",
                    i + 1,
                    filter.filter_type,
                    filter.value
                ))
                .style(Style::new().light_magenta()),
            );
        }
    }

    lines.extend([
        Line::from(""),
        Line::from("⚡ Actions:").style(Style::new().bold().cyan()),
        Line::from("   [a] Add filter    [d] Delete filter").style(Style::new().cyan()),
        Line::from("   [g] Filter ≥      [l] Filter ≤").style(Style::new().cyan()),
        Line::from("   [r] Calculate     [c] Clear filters").style(Style::new().cyan()),
        Line::from(""),
        Line::from("🎮 Navigation:").style(Style::new().bold().green()),
        Line::from("   1,2,s,e,v Edit fields").style(Style::new().green()),
        Line::from("   ESC/Enter Exit edit mode").style(Style::new().green()),
        Line::from("   ↑↓ Navigate results").style(Style::new().green()),
    ]);

    let mut text = Text::from(lines);
    if let Some(err) = &state.error {
        text.lines.push(Line::from(""));
        text.lines
            .push(Line::from("❌ Error:").style(Style::new().bold().red()));
        text.lines
            .push(Line::from(format!("   {}", err)).style(Style::new().red()));
    }

    Paragraph::new(text).wrap(Wrap { trim: true })
}
