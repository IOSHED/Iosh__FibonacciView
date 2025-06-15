mod event;
mod state;

use ratatui::DefaultTerminal;
use std::io;
use crate::ui;
pub use self::state::{AppState, InputMode, FilterType, Filter};
pub use self::event::handle_events;

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