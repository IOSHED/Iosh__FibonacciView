mod event;
mod state;

pub use self::state::{AppState, Filter, FilterType, InputMode};
use crate::app::event::EventHandler;
use crate::ui;
use ratatui::DefaultTerminal;
use std::io;

pub struct TerminalApp {
    terminal: DefaultTerminal,
    state: AppState,
}

impl TerminalApp {
    pub fn new() -> Self {
        Self {
            terminal: ratatui::init(),
            state: AppState::new(),
        }
    }

    pub async fn run(&mut self) -> io::Result<()> {
        loop {
            if EventHandler::handle(&mut self.state).await? {
                break;
            }
            self.state.update_progress_bar().await;
            self.terminal.draw(|f| ui::draw(f, &mut self.state))?;
        }
        Ok(())
    }

    pub fn restore(self) {
        ratatui::restore();
    }
}
