use crate::app::state::AppState;
use crate::ui::input_panel::result_renderer::ResultRenderer;
use ratatui::widgets::Paragraph;

mod list_styles;
mod result_renderer;

pub fn render(state: &AppState) -> Paragraph {
    ResultRenderer::new(state).render()
}
