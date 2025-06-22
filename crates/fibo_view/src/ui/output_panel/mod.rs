use crate::app::state::AppState;
use crate::ui::output_panel::result_renderer::ResultRenderer;
use ratatui::layout::Rect;
use ratatui::widgets::List;

mod list_styles;
mod result_renderer;


pub fn render(state: &mut AppState, area: Rect) -> List {
    let viewport_height = area.height.saturating_sub(2) as usize;
    state.output.viewport_size = viewport_height.max(1);

    ResultRenderer::new(state).render()
}
