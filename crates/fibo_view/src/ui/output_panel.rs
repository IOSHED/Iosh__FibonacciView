use ratatui::{
    prelude::*,
};
use ratatui::widgets::{Block, Wrap};

use ratatui::{
    prelude::*,
    text::Line,
    widgets::Paragraph,
};

use crate::app::AppState;

pub fn render(state: &mut AppState) -> Paragraph {
    if state.results.is_empty() {
        // Форматируем пустое состояние как многострочный текст
        let text = Text::from(vec![
            Line::from(""),
            Line::from("   🎯 No results yet").style(Style::new().italic().dark_gray()),
            Line::from(""),
            Line::from("   💡 Tips:").style(Style::new().bold().yellow()),
            Line::from("   • Set your starting numbers").style(Style::new().yellow()),
            Line::from("   • Define the range (start-end)").style(Style::new().yellow()),
            Line::from("   • Add filters if needed").style(Style::new().yellow()),
            Line::from("   • Press [r] to calculate!").style(Style::new().yellow()),
            Line::from(""),
            Line::from("   🔢 Example Fibonacci:").style(Style::new().bold().cyan()),
            Line::from("   0, 1, 1, 2, 3, 5, 8, 13, 21...").style(Style::new().cyan()),
        ]);

        return Paragraph::new(text)
            .block(Block::default())
            .alignment(Alignment::Left);
    }

    let numbers_str = state
        .results
        .iter()
        .enumerate()
        .map(|(i, num)| {
            // Форматируем число с учетом выделения
            let num_str = if num.to_string().len() > 50 {
                format!("{}...", &num.to_string()[..47])
            } else {
                num.to_string()
            };

            if i == state.list_state.selected().unwrap_or(0) {
                format!("[{}]", num_str).bold().yellow().to_string()
            } else if i % 2 == 0 {
                num_str.white().to_string()
            } else {
                num_str.light_blue().to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    Paragraph::new(numbers_str)
        .block(Block::default())
        .wrap(Wrap { trim: true })
}