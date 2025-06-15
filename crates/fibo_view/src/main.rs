mod app;
mod domain;
mod terminal_app;
mod ui;

use crate::terminal_app::TerminalApp;

fn main() -> std::io::Result<()> {
    let mut terminal = TerminalApp::new();

    terminal.run()?;

    terminal.restore();

    Ok(())
}
