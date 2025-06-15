use super::state::AppState;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub fn handle_events(state: &mut AppState) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => handle_keys(key, state),
        _ => Ok(false),
    }
}

fn handle_keys(key: KeyEvent, state: &mut AppState) -> std::io::Result<bool> {
    match state.input_mode {
        super::InputMode::Normal => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('1') => state.input_mode = super::InputMode::Start1,
            KeyCode::Char('2') => state.input_mode = super::InputMode::Start2,
            KeyCode::Char('s') => state.input_mode = super::InputMode::RangeStart,
            KeyCode::Char('e') => state.input_mode = super::InputMode::RangeEnd,
            KeyCode::Char('v') => state.input_mode = super::InputMode::FilterValue,
            KeyCode::Char('a') => state.add_filter(),
            KeyCode::Char('r') => state.calculate(),
            KeyCode::Char('c') => state.filters.clear(),
            KeyCode::Char('d') => state.delete_filter(),
            KeyCode::Up => state.scroll_results(-1),
            KeyCode::Down => state.scroll_results(1),
            _ => {}
        },
        _ => match key.code {
            KeyCode::Enter => state.input_mode = super::InputMode::Normal,
            KeyCode::Char(c) => handle_char_input(c, state),
            KeyCode::Backspace => handle_backspace(state),
            KeyCode::Esc => state.input_mode = super::InputMode::Normal,
            _ => {}
        },
    }
    Ok(false)
}

fn handle_char_input(c: char, state: &mut AppState) {
    match state.input_mode {
        super::InputMode::Start1 => state.start1.push(c),
        super::InputMode::Start2 => state.start2.push(c),
        super::InputMode::RangeStart => state.range_start.push(c),
        super::InputMode::RangeEnd => state.range_end.push(c),
        super::InputMode::FilterValue => state.filter_value.push(c),
        _ => {}
    }
}

fn handle_backspace(state: &mut AppState) {
    match state.input_mode {
        super::InputMode::Start1 => {
            state.start1.pop();
        }
        super::InputMode::Start2 => {
            state.start2.pop();
        }
        super::InputMode::RangeStart => {
            state.range_start.pop();
        }
        super::InputMode::RangeEnd => {
            state.range_end.pop();
        }
        super::InputMode::FilterValue => {
            state.filter_value.pop();
        }
        _ => {}
    }
}
