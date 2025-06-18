use crate::app::TerminalApp;

mod app;
mod domain;
mod ui;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = TerminalApp::new();

    terminal.run().await?;

    terminal.restore();

    Ok(())
}
