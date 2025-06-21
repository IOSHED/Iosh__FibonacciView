use std::fs;
use fibo_view::TerminalApp;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let output_dir = "../../target/profiling";
    fs::create_dir_all(output_dir).expect("Failed to create directory");

    let _profiler = dhat::Profiler::builder()
        .file_name(format!("{}/dhat-heap-view.json", output_dir))
        .build();

    let mut terminal = TerminalApp::new();

    terminal.run().await?;

    terminal.restore();

    Ok(())
}
