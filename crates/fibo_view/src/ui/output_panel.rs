use ratatui::{
    prelude::*,
    widgets::{List, ListItem},
};
use ratatui::widgets::Block;
use crate::app::AppState;

pub fn render(state: &mut AppState) -> List {
    if state.results.is_empty() {
        let empty_items = vec![
            ListItem::new(""),
            ListItem::new("   🎯 No results yet").style(Style::new().italic().dark_gray()),
            ListItem::new(""),
            ListItem::new("   💡 Tips:").style(Style::new().bold().yellow()),
            ListItem::new("   • Set your starting numbers").style(Style::new().yellow()),
            ListItem::new("   • Define the range (start-end)").style(Style::new().yellow()),
            ListItem::new("   • Add filters if needed").style(Style::new().yellow()),
            ListItem::new("   • Press [r] to calculate!").style(Style::new().yellow()),
            ListItem::new(""),
            ListItem::new("   🔢 Example Fibonacci:").style(Style::new().bold().cyan()),
            ListItem::new("   0, 1, 1, 2, 3, 5, 8, 13, 21...").style(Style::new().cyan()),
        ];

        return List::new(empty_items)
            .highlight_style(Style::new().bold().yellow())
            .block(Block::default());
    }

    let items: Vec<_> = state
        .results
        .iter()
        .enumerate()
        .map(|(i, num)| {
            let mut formatted = if num.to_string().len() > 50 {
                format!("{}...",  &num.to_string()[..47])
            } else {
                format!("{}", num)
            };

            if i == state.list_state.selected().unwrap_or(0) {
                formatted = format!("[{}]", formatted).bold().yellow().to_string();
            }

            Line::from(formatted)
                .style(if i % 2 == 0 {
                    Style::new().white()
                } else {
                    Style::new().light_blue()
                })

        })
        .collect();

    List::new(items)
        .highlight_style(Style::new().bold().yellow().on_dark_gray())
        .block(Block::default())
}