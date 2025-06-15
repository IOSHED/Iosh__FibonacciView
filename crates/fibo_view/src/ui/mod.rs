mod input_panel;
mod output_panel;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::*,
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::app::AppState;

pub fn draw(frame: &mut Frame, state: &mut AppState) {
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    let title_text = "🔢 FIBONACCI CALCULATOR 🔢";
    let title_line = Line::from(title_text)
        .centered()
        .style(Style::new().bold().cyan());

    let subtitle_line = Line::from("Interactive Fibonacci Sequence Generator")
        .centered()
        .style(Style::new().italic().light_blue());

    let status_text = format!(
        "📊 Calculations: {} | 📋 Results: {} | 🔍 Filters: {} | ⌨️  Press 'q' to quit",
        state.count_use,
        state.results.len(),
        state.filters.len()
    );
    let status_line = Line::from(status_text)
        .centered()
        .style(Style::new().bold().green());

    let input_block = Block::bordered()
        .title(" 📝 Input Parameters ")
        .title_style(Style::new().bold().light_red())
        .border_style(Style::new().light_red());

    let output_block = Block::bordered()
        .title(" 📊 Fibonacci Results ")
        .title_style(Style::new().bold().cyan())
        .border_style(Style::new().cyan());

    frame.render_widget(
        Block::bordered()
            .title_style(Style::new().bold().cyan())
            .border_style(Style::new().cyan()),
        title_area,
    );
    let title_inner = Block::bordered().inner(title_area);
    frame.render_widget(
        Paragraph::new(vec![title_line, subtitle_line]).centered(),
        title_inner,
    );

    frame.render_widget(
        Block::bordered()
            .title_style(Style::new().bold().green())
            .border_style(Style::new().green()),
        status_area,
    );
    let status_inner = Block::bordered().inner(status_area);
    frame.render_widget(Paragraph::new(status_line).centered(), status_inner);

    frame.render_widget(&input_block, left_area);
    frame.render_widget(&output_block, right_area);

    let inner_left = input_block.inner(left_area);
    frame.render_widget(input_panel::render(state), inner_left);

    let inner_right = output_block.inner(right_area);
    frame.render_widget(output_panel::render(state), inner_right);
}
