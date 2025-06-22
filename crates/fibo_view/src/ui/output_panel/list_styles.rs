use ratatui::prelude::Stylize;
use ratatui::style::Style;

#[derive(Clone)]
pub struct ListStyles {
    pub empty_text: Style,
    pub tips_header: Style,
    pub tip_item: Style,
    pub example_header: Style,
    pub example_text: Style,
    pub highlight: Style,
    pub selected_item: Style,
    pub calculating_text: Style,
    pub progress_text: Style,
    pub note_list_item: Style,
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
            note_list_item: Style::new().italic().dark_gray(),
        }
    }
}
