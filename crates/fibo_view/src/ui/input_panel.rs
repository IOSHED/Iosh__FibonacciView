use crate::app::{AppState, InputMode};
use ratatui::{
    prelude::*,
    text::{Line, Text},
    widgets::{Paragraph, Wrap},
};

struct PanelStyles {
    active_input: Style,
    inactive_start: Style,
    inactive_range: Style,
    inactive_filter: Style,
    filter_header: Style,
    no_filter: Style,
    filter_item: Style,
    action_header: Style,
    action_item: Style,
    nav_header: Style,
    nav_item: Style,
    error_header: Style,
    error_text: Style,
}

impl Default for PanelStyles {
    fn default() -> Self {
        Self {
            active_input: Style::new().bold().yellow(),
            inactive_start: Style::new().white(),
            inactive_range: Style::new().light_blue(),
            inactive_filter: Style::new().light_green(),
            filter_header: Style::new().bold().magenta(),
            no_filter: Style::new().italic().dark_gray(),
            filter_item: Style::new().light_magenta(),
            action_header: Style::new().bold().cyan(),
            action_item: Style::new().cyan(),
            nav_header: Style::new().bold().green(),
            nav_item: Style::new().green(),
            error_header: Style::new().bold().red(),
            error_text: Style::new().red(),
        }
    }
}

pub fn render(state: &AppState) -> Paragraph {
    let styles = PanelStyles::default();
    let mut lines = Vec::new();

    append_input_fields(&mut lines, state, &styles);
    append_filter_section(&mut lines, state, &styles);
    append_action_section(&mut lines, &styles);
    append_navigation_section(&mut lines, &styles);

    let mut text = Text::from(lines);
    append_error_section(&mut text, state, &styles);

    Paragraph::new(text).wrap(Wrap { trim: true })
}

fn get_field_style(current_mode: &InputMode, target_mode: InputMode, styles: &PanelStyles) -> Style {
    if *current_mode == target_mode {
        styles.active_input
    } else {
        match target_mode {
            InputMode::Start1 | InputMode::Start2 => styles.inactive_start,
            InputMode::RangeStart | InputMode::RangeEnd => styles.inactive_range,
            InputMode::FilterValue => styles.inactive_filter,
            _ => Style::default(),
        }
    }
}

fn append_input_fields(lines: &mut Vec<Line>, state: &AppState, styles: &PanelStyles) {
    lines.extend([
        Line::from(""),
        Line::from(format!("🔢 Start Number 1 [1]: {}", state.input.start1))
            .style(get_field_style(&state.input_mode, InputMode::Start1, styles)),
        Line::from(format!("🔢 Start Number 2 [2]: {}", state.input.start2))
            .style(get_field_style(&state.input_mode, InputMode::Start2, styles)),
        Line::from(""),
        Line::from(format!("📍 Range Start [s]: {}", state.input.range_start))
            .style(get_field_style(&state.input_mode, InputMode::RangeStart, styles)),
        Line::from(format!("📍 Range End [e]: {}", state.input.range_end))
            .style(get_field_style(&state.input_mode, InputMode::RangeEnd, styles)),
        Line::from(""),
        Line::from(format!(
            "🔍 Filter Value [v]: {}{}",
            state.filters.filter_type, state.input.filter_value
        ))
        .style(get_field_style(&state.input_mode, InputMode::FilterValue, styles)),
        Line::from(""),
        Line::from("🔍 Active Filters:").style(styles.filter_header),
    ]);
}

fn append_filter_section(lines: &mut Vec<Line>, state: &AppState, styles: &PanelStyles) {
    if state.filters.filters.is_empty() {
        lines.push(Line::from("   (No filters applied)").style(styles.no_filter));
    } else {
        for (i, filter) in state.filters.filters.iter().enumerate() {
            lines.push(
                Line::from(format!(
                    "   {}. {} {}",
                    i + 1,
                    filter.filter_type,
                    filter.value
                ))
                .style(styles.filter_item),
            );
        }
    }
}

fn append_action_section(lines: &mut Vec<Line>, styles: &PanelStyles) {
    lines.extend([
        Line::from(""),
        Line::from("⚡ Actions:").style(styles.action_header),
        Line::from("   [a] Add filter    [d] Delete filter").style(styles.action_item),
        Line::from("   [g] Filter ≥      [l] Filter ≤").style(styles.action_item),
        Line::from("   [r] Calculate     [c] Clear filters").style(styles.action_item),
    ]);
}

fn append_navigation_section(lines: &mut Vec<Line>, styles: &PanelStyles) {
    lines.extend([
        Line::from(""),
        Line::from("🎮 Navigation:").style(styles.nav_header),
        Line::from("   1,2,s,e,v Edit fields").style(styles.nav_item),
        Line::from("   ESC/Enter Exit edit mode").style(styles.nav_item),
        Line::from("   ↑↓ Navigate results").style(styles.nav_item),
    ]);
}

fn append_error_section(text: &mut Text, state: &AppState, styles: &PanelStyles) {
    if let Some(err) = &state.error {
        text.lines.extend([
            Line::from(""),
            Line::from("❌ Error:").style(styles.error_header),
            Line::from(format!("   {}", err)).style(styles.error_text),
        ]);
    }
}
