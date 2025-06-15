mod event;
mod state;

pub use self::event::handle_events;
pub use self::state::{AppState, Filter, FilterType, InputMode};
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

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            self.terminal
                .draw(|f| ui::draw(f, &mut self.state))
                .expect("failed to draw frame");

            if handle_events(&mut self.state)? {
                break Ok(());
            }
        }
    }

    pub fn restore(self) {
        ratatui::restore();
    }
}
