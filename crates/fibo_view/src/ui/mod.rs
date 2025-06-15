mod input_panel;
mod output_panel;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::{Span, Style},
    style::Stylize,
    text::Line,
    widgets::Block,
};

use crate::app::AppState;

pub fn draw(frame: &mut Frame, state: &mut AppState) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    // Title
    let mut main_title = Line::from("FIBONACCI").centered().cyan();
    main_title.push_span(Span::raw(" VIEW").style(Style::new().light_red()));

    // Status
    let status_text = format!(
        " Uses: {} | Elements: {} | Filters: {} ",
        state.count_use,
        state.results.len(),
        state.filters.len()
    );
    let status_title = Line::from(status_text);

    // Create blocks
    let input_block = Block::bordered().title(" Input ").light_red();
    let output_block = Block::bordered().title(" Output ").cyan();

    // Draw title and status
    frame.render_widget(Block::bordered().title(main_title).dark_gray(), title_area);
    frame.render_widget(
        Block::bordered().title(status_title).dark_gray(),
        status_area,
    );

    // Render widgets by reference
    frame.render_widget(&input_block, left_area);
    frame.render_widget(&output_block, right_area);

    // Draw input panel content
    let inner_left = input_block.inner(left_area);
    frame.render_widget(input_panel::render(state), inner_left);

    // Draw output panel content
    let inner_right = output_block.inner(right_area);
    frame.render_widget(output_panel::render(state), inner_right);
}
