use crate::app::{AppState, InputMode};
use ratatui::{
    prelude::*,
    text::{Line, Text},
    widgets::{Paragraph, Wrap},
};

use crate::ui::input_panel::list_styles::ListStyles;

pub struct ResultRenderer<'a> {
    pub state: &'a AppState,
    pub styles: ListStyles,
}

impl<'a> ResultRenderer<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self {
            state,
            styles: ListStyles::default(),
        }
    }

    pub fn render(self) -> Paragraph<'a> {
        let mut lines = Vec::new();

        self.append_input_fields(&mut lines);
        self.append_filter_section(&mut lines);
        self.append_action_section(&mut lines);
        self.append_navigation_section(&mut lines);

        let mut text = Text::from(lines);
        self.append_error_section(&mut text);

        Paragraph::new(text).wrap(Wrap { trim: true })
    }

    fn get_field_style(&self, current_mode: &InputMode, target_mode: InputMode) -> Style {
        if *current_mode == target_mode {
            self.styles.active_input
        } else {
            match target_mode {
                InputMode::Start1 | InputMode::Start2 => self.styles.inactive_start,
                InputMode::RangeStart | InputMode::RangeEnd => self.styles.inactive_range,
                InputMode::FilterValue => self.styles.inactive_filter,
                _ => Style::default(),
            }
        }
    }

    fn append_input_fields(&self, lines: &mut Vec<Line>) {
        lines.extend([
            Line::from(""),
            Line::from(format!(
                "🔢 Start Number 1 [1]: {}",
                self.state.input.start1
            ))
            .style(self.get_field_style(&self.state.input_mode, InputMode::Start1)),
            Line::from(format!(
                "🔢 Start Number 2 [2]: {}",
                self.state.input.start2
            ))
            .style(self.get_field_style(&self.state.input_mode, InputMode::Start2)),
            Line::from(""),
            Line::from(format!(
                "📍 Range Start [s]: {}",
                self.state.input.range_start
            ))
            .style(self.get_field_style(&self.state.input_mode, InputMode::RangeStart)),
            Line::from(format!("📍 Range End [e]: {}", self.state.input.range_end))
                .style(self.get_field_style(&self.state.input_mode, InputMode::RangeEnd)),
            Line::from(""),
            Line::from(format!(
                "🔍 Filter Value [v]: {}{}",
                self.state.filters.filter_type, self.state.input.filter_value
            ))
            .style(self.get_field_style(&self.state.input_mode, InputMode::FilterValue)),
            Line::from(""),
            Line::from("🔍 Active Filters:").style(self.styles.filter_header),
        ]);
    }

    fn append_filter_section(&self, lines: &mut Vec<Line>) {
        if self.state.filters.filters.is_empty() {
            lines.push(Line::from("   (No filters applied)").style(self.styles.no_filter));
        } else {
            for (i, filter) in self.state.filters.filters.iter().enumerate() {
                lines.push(
                    Line::from(format!(
                        "   {}. {} {}",
                        i + 1,
                        filter.filter_type,
                        filter.value
                    ))
                    .style(self.styles.filter_item),
                );
            }
        }
    }

    fn append_action_section(&self, lines: &mut Vec<Line>) {
        lines.extend([
            Line::from(""),
            Line::from("⚡ Actions:").style(self.styles.action_header),
            Line::from("   [a] Add filter    [d] Delete filter").style(self.styles.action_item),
            Line::from("   [g] Filter ≥      [l] Filter ≤").style(self.styles.action_item),
            Line::from("   [r] Calculate     [c] Clear filters").style(self.styles.action_item),
        ]);
    }

    fn append_navigation_section(&self, lines: &mut Vec<Line>) {
        lines.extend([
            Line::from(""),
            Line::from("🎮 Navigation:").style(self.styles.nav_header),
            Line::from("   1,2,s,e,v Edit fields").style(self.styles.nav_item),
            Line::from("   ESC/Enter Exit edit mode").style(self.styles.nav_item),
            Line::from("   ↑↓ Navigate results").style(self.styles.nav_item),
        ]);
    }

    fn append_error_section(&self, text: &mut Text) {
        if let Some(err) = &self.state.error {
            text.lines.extend([
                Line::from(""),
                Line::from("❌ Error:").style(self.styles.error_header),
                Line::from(format!("   {}", err)).style(self.styles.error_text),
            ]);
        }
    }
}
