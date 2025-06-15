use crate::app::TerminalApp;

mod app;
mod domain;
mod ui;

fn main() -> std::io::Result<()> {
    let mut terminal = TerminalApp::new();

    terminal.run()?;

    terminal.restore();

    Ok(())
}
