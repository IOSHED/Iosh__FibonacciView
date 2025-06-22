use ratatui::widgets::Paragraph;
use crate::app::AppState;
use crate::ui::input_panel::result_renderer::ResultRenderer;

mod list_styles;
mod result_renderer;

pub fn render(state: &AppState) -> Paragraph {
    ResultRenderer::new(state).render()
}
