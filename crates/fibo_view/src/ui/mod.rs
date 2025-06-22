mod input_panel;
mod output_panel;
mod list_styles;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::*,
    text::Line,
    widgets::{Block, Gauge, Paragraph},
    Frame,
};
use list_styles::ListStyles;
use crate::app::AppState;

struct LayoutAreas {
    title: Rect,
    _main: Rect,
    status: Rect,
    left: Rect,
    right: Rect,
    progress: Rect,
}

pub fn draw(frame: &mut Frame, state: &mut AppState) {
    let styles = ListStyles::default();
    let areas = create_layout(frame.area());

    render_title_section(frame, &areas, &styles);
    render_status_section(frame, &areas, state, &styles);
    render_progress_section(frame, &areas, state, &styles);
    render_main_sections(frame, &areas, state, &styles);
}

fn create_layout(area: Rect) -> LayoutAreas {
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(3), // Progress bar area
        Constraint::Min(0),
        Constraint::Length(3),
    ]);
    let [title_area, progress_area, main_area, status_area] = vertical.areas(area);

    let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    LayoutAreas {
        title: title_area,
        _main: main_area,
        status: status_area,
        left: left_area,
        right: right_area,
        progress: progress_area,
    }
}

fn render_title_section(frame: &mut Frame, areas: &LayoutAreas, styles: &ListStyles) {
    let style = styles.title;
    let title_block = Block::bordered().title_style(style).border_style(style);
    frame.render_widget(&title_block, areas.title);

    let title_content = vec![
        Line::from("🔢 FIBONACCI CALCULATOR 🔢")
            .centered()
            .style(styles.title),
        Line::from("Interactive Fibonacci Sequence Generator")
            .centered()
            .style(styles.subtitle),
    ];

    let title_inner = Block::bordered().inner(areas.title);
    frame.render_widget(Paragraph::new(title_content).centered(), title_inner);
}

fn render_status_section(
    frame: &mut Frame, areas: &LayoutAreas, state: &AppState, styles: &ListStyles,
) {
    let style = styles.status;
    let status_block = Block::bordered().title_style(style).border_style(style);
    frame.render_widget(&status_block, areas.status);

    let position_info = if !state.output.results.is_empty() {
        let selected = state.output.list_state.selected().unwrap_or(0);
        let total = state.output.results.len();
        format!("{}/{}", selected + 1, total)
    } else {
        String::from("0/0")
    };

    let status_text = format!(
        "📊 Calculations: {} | 📍 Position: {} | 🔍 Filters: {} | ⌨️  Press 'q' to quit",
        state.count_use,
        position_info,
        state.filters.filters.len(),
    );

    let status_line = Line::from(status_text).centered().style(styles.status);

    let status_inner = Block::bordered().inner(areas.status);
    frame.render_widget(Paragraph::new(status_line).centered(), status_inner);
}

fn render_main_sections(
    frame: &mut Frame, areas: &LayoutAreas, state: &mut AppState, styles: &ListStyles,
) {
    let input_block = Block::bordered()
        .title(" 📝 Input Parameters ")
        .title_style(styles.input_block)
        .border_style(styles.input_block);

    let output_block = Block::bordered()
        .title(" 📊 Fibonacci Results ")
        .title_style(styles.output_block)
        .border_style(styles.output_block);

    frame.render_widget(&input_block, areas.left);
    frame.render_widget(&output_block, areas.right);

    let inner_left = input_block.inner(areas.left);
    let inner_right = output_block.inner(areas.right);

    frame.render_widget(input_panel::render(state), inner_left);
    frame.render_widget(output_panel::render(state, inner_right), inner_right);
}

fn render_progress_section(
    frame: &mut Frame, areas: &LayoutAreas, state: &AppState, styles: &ListStyles,
) {
    if let Some(progress) = state.output.progress {
        render_progress_bar_with_progress(frame, areas, styles, progress);
        return;
    }
    let empty_block = Block::bordered()
        .title(" 💤 Ready ")
        .title_style(Style::new().dim())
        .border_style(Style::new().dim());

    let empty_inner = empty_block.inner(areas.progress);
    frame.render_widget(empty_block, areas.progress);

    let ready_text = Line::from("Press 'r' to start calculation")
        .centered()
        .style(Style::new().dim());
    frame.render_widget(Paragraph::new(ready_text).centered(), empty_inner);

}

fn render_progress_bar_with_progress(frame: &mut Frame, areas: &LayoutAreas, styles: &ListStyles, progress: u8) {
    let progress_block = Block::bordered()
        .title(" ⏳ Calculation Progress ")
        .title_style(styles.progress_bar)
        .border_style(styles.progress_bar);

    frame.render_widget(&progress_block, areas.progress);

    let progress_inner = progress_block.inner(areas.progress);

    let progress_char = match progress {
        0..=25 => "█",
        26..=50 => "▓",
        51..=75 => "▒",
        _ => "░",
    };

    let gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(styles.progress_bar)
        .percent(progress as u16)
        .label(format!(
            "{}% {} Computing Fibonacci...",
            progress, progress_char
        ));

    frame.render_widget(gauge, progress_inner);
}
