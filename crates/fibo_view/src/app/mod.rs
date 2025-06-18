mod event;
mod state;

pub use self::event::handle_events;
pub use self::state::{AppState, Filter, FilterType, InputMode};
use crate::ui;
use ratatui::DefaultTerminal;
use std::io;
use std::time::Duration;
use tokio;
use tokio::time::interval;

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
        let mut tick_interval = interval(Duration::from_millis(100));

        loop {
            tokio::select! {
                _ = tick_interval.tick() => {
                    self.state.update_progress_bar().await;
                    self.terminal.draw(|f| ui::draw(f, &mut self.state))?;
                }
                exit = handle_events(&mut self.state) => {
                    if exit? {
                        break;
                    }
                    self.terminal.draw(|f| ui::draw(f, &mut self.state))?;
                }
            }
        }
        Ok(())
    }

    pub fn restore(self) {
        ratatui::restore();
    }
}
