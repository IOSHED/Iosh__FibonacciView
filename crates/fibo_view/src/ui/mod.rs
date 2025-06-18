mod input_panel;
mod output_panel;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    prelude::*,
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::app::AppState;

struct UiStyles {
    title: Style,
    subtitle: Style,
    status: Style,
    input_block: Style,
    output_block: Style,
}

impl Default for UiStyles {
    fn default() -> Self {
        Self {
            title: Style::new().bold().cyan(),
            subtitle: Style::new().italic().light_blue(),
            status: Style::new().bold().green(),
            input_block: Style::new().bold().light_red(),
            output_block: Style::new().bold().cyan(),
        }
    }
}

struct LayoutAreas {
    title: Rect,
    _main: Rect,
    status: Rect,
    left: Rect,
    right: Rect,
}

pub fn draw(frame: &mut Frame, state: &mut AppState) {
    let styles = UiStyles::default();
    let areas = create_layout(frame.area());

    render_title_section(frame, &areas, &styles);
    render_status_section(frame, &areas, state, &styles);
    render_main_sections(frame, &areas, state, &styles);
}

fn create_layout(area: Rect) -> LayoutAreas {
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ]);
    let [title_area, main_area, status_area] = vertical.areas(area);

    let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    LayoutAreas {
        title: title_area,
        _main: main_area,
        status: status_area,
        left: left_area,
        right: right_area,
    }
}

fn create_bordered_block(style: Style) -> Block<'static> {
    Block::bordered().title_style(style).border_style(style)
}

fn render_title_section(frame: &mut Frame, areas: &LayoutAreas, styles: &UiStyles) {
    let title_block = create_bordered_block(styles.title);
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
    frame: &mut Frame, areas: &LayoutAreas, state: &AppState, styles: &UiStyles,
) {
    let status_block = create_bordered_block(styles.status);
    frame.render_widget(&status_block, areas.status);

    let status_text = format!(
        "📊 Calculations: {} | 📋 Results: {} | 🔍 Filters: {} | ⌨️  Press 'q' to quit",
        state.count_use,
        state.output.results.len(),
        state.filters.filters.len()
    );

    let status_line = Line::from(status_text).centered().style(styles.status);

    let status_inner = Block::bordered().inner(areas.status);
    frame.render_widget(Paragraph::new(status_line).centered(), status_inner);
}

fn render_main_sections(
    frame: &mut Frame, areas: &LayoutAreas, state: &mut AppState, styles: &UiStyles,
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
    frame.render_widget(output_panel::render(state), inner_right);
}
