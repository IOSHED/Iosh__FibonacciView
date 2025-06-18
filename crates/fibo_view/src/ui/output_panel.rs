use crate::app::AppState;
use num_bigint::BigInt;
use ratatui::prelude::Stylize;
use ratatui::{
    style::Style,
    widgets::{Block, List, ListItem},
};

struct ListStyles {
    empty_text: Style,
    tips_header: Style,
    tip_item: Style,
    example_header: Style,
    example_text: Style,
    highlight: Style,
    selected_item: Style,
    calculating_text: Style,
    progress_text: Style,
}

impl Default for ListStyles {
    fn default() -> Self {
        Self {
            empty_text: Style::new().italic().dark_gray(),
            tips_header: Style::new().bold().yellow(),
            tip_item: Style::new().yellow(),
            example_header: Style::new().bold().cyan(),
            example_text: Style::new().cyan(),
            highlight: Style::new().bold().yellow(),
            selected_item: Style::new().bold().yellow().on_dark_gray(),
            calculating_text: Style::new().bold().green(),
            progress_text: Style::new().italic().light_blue(),
        }
    }
}

struct ResultRenderer<'a> {
    state: &'a AppState,
    styles: ListStyles,
}

impl<'a> ResultRenderer<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self {
            state,
            styles: ListStyles::default(),
        }
    }

    pub fn render(&self) -> List<'a> {
        if self.state.output.progress.is_some() {
            self.render_calculating_state()
        } else if self.state.output.results.is_empty() {
            self.render_empty_state()
        } else {
            self.render_results()
        }
    }

    fn render_empty_state(&self) -> List<'a> {
        let empty_items = self.create_empty_state_items();

        List::new(empty_items)
            .highlight_style(self.styles.highlight)
            .block(Block::default())
    }

    fn create_empty_state_items(&self) -> Vec<ListItem<'a>> {
        vec![
            ListItem::new(""),
            ListItem::new("   🎯 No results yet").style(self.styles.empty_text),
            ListItem::new(""),
            ListItem::new("   💡 Tips:").style(self.styles.tips_header),
            ListItem::new("   • Set your starting numbers").style(self.styles.tip_item),
            ListItem::new("   • Define the range (start-end)").style(self.styles.tip_item),
            ListItem::new("   • Add filters if needed").style(self.styles.tip_item),
            ListItem::new("   • Press [r] to calculate!").style(self.styles.tip_item),
            ListItem::new(""),
            ListItem::new("   🔢 Example Fibonacci:").style(self.styles.example_header),
            ListItem::new("   0, 1, 1, 2, 3, 5, 8, 13, 21...").style(self.styles.example_text),
        ]
    }

    fn render_results(&self) -> List<'a> {
        let items = self.format_result_items();

        List::new(items)
            .highlight_style(self.styles.selected_item)
            .block(Block::default())
    }

    fn render_calculating_state(&self) -> List<'a> {
        let progress = self.state.output.progress.unwrap_or(0);
        let calculating_items = self.create_calculating_state_items(progress);

        List::new(calculating_items)
            .highlight_style(self.styles.highlight)
            .block(Block::default())
    }

    fn create_calculating_state_items(&self, progress: u8) -> Vec<ListItem<'a>> {
        let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let spinner_index = (progress / 10) as usize % spinner_chars.len();
        let spinner = spinner_chars[spinner_index];

        vec![
            ListItem::new(""),
            ListItem::new(format!("   {} Calculating Fibonacci sequence...", spinner))
                .style(self.styles.calculating_text),
            ListItem::new(""),
            ListItem::new(format!("   📊 Progress: {}%", progress))
                .style(self.styles.progress_text),
            ListItem::new(""),
            ListItem::new("   ⏳ Please wait while we compute the numbers...")
                .style(self.styles.progress_text),
            ListItem::new(""),
            ListItem::new("   💡 Tip: The UI remains responsive during calculation!")
                .style(self.styles.tip_item),
            ListItem::new(""),
            ListItem::new("   🔄 You can still navigate and interact with the interface")
                .style(self.styles.tip_item),
        ]
    }

    fn format_result_items(&self) -> Vec<ListItem<'a>> {
        self.state
            .output
            .results
            .iter()
            .enumerate()
            .map(|(i, num)| self.format_result_item(i, num))
            .collect()
    }

    fn format_result_item(&self, index: usize, num: &'a BigInt) -> ListItem<'a> {
        let formatted = if num.to_string().len() > 50 {
            format!("{}...", &num.to_string()[..47])
        } else {
            num.to_string()
        };
        let formatted = if index == self.state.output.list_state.selected().unwrap_or(0) {
            format!("[{}]", formatted).bold().yellow().to_string()
        } else {
            formatted
        };


        let style = if index % 2 == 0 {
            Style::new().white()
        } else {
            Style::new().light_blue()
        };

        ListItem::new(formatted).style(style)
    }
}

pub fn render(state: &AppState) -> List {
    ResultRenderer::new(state).render()
}
