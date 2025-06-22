use ratatui::prelude::{Style, Stylize};

pub struct ListStyles {
    pub title: Style,
    pub subtitle: Style,
    pub status: Style,
    pub input_block: Style,
    pub output_block: Style,
    pub progress_bar: Style,
}

impl Default for ListStyles {
    fn default() -> Self {
        Self {
            title: Style::new().bold().cyan(),
            subtitle: Style::new().italic().light_blue(),
            status: Style::new().bold().green(),
            input_block: Style::new().bold().light_red(),
            output_block: Style::new().bold().cyan(),
            progress_bar: Style::new().bold().green(),
        }
    }
}