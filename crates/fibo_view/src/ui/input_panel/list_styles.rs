use ratatui::prelude::Stylize;
use ratatui::style::Style;

#[derive(Clone)]
pub struct ListStyles {
    pub active_input: Style,
    pub inactive_start: Style,
    pub inactive_range: Style,
    pub inactive_filter: Style,
    pub filter_header: Style,
    pub no_filter: Style,
    pub filter_item: Style,
    pub action_header: Style,
    pub action_item: Style,
    pub nav_header: Style,
    pub nav_item: Style,
    pub error_header: Style,
    pub error_text: Style,
}

impl Default for ListStyles {
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
