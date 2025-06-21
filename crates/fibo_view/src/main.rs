use fibo_view::TerminalApp;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = TerminalApp::new();

    terminal.run().await?;

    terminal.restore();

    Ok(())
}
