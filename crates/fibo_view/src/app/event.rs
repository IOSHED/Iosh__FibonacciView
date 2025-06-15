use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crate::app::{AppState, FilterType};
use crate::app::state::InputMode;

pub fn handle_events(state: &mut AppState) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => handle_keys(key, state),
        _ => Ok(false),
    }
}

fn handle_keys(key: KeyEvent, state: &mut AppState) -> std::io::Result<bool> {
    match state.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('1') => {
                state.input_mode = InputMode::Start1;
                state.start1.clear();
            }
            KeyCode::Char('2') => {
                state.input_mode = InputMode::Start2;
                state.start2.clear();
            }
            KeyCode::Char('s') => {
                state.input_mode = InputMode::RangeStart;
                state.range_start.clear();
            }
            KeyCode::Char('g') => {
                state.filter_type = FilterType::Ge;
            }
            KeyCode::Char('l') => {
                state.filter_type = FilterType::Le;
            }
            KeyCode::Char('e') => {
                state.input_mode = InputMode::RangeEnd;
                state.range_end.clear();
            }
            KeyCode::Char('v') => {
                state.input_mode = InputMode::FilterValue;
                state.filter_value.clear();
            }
            KeyCode::Char('a') => state.add_filter(),
            KeyCode::Char('r') => state.calculate(),
            KeyCode::Char('c') => state.filters.clear(),
            KeyCode::Char('d') => state.delete_filter(),
            KeyCode::Up => state.scroll_results(-1),
            KeyCode::Down => state.scroll_results(1),
            _ => {}
        },
        _ => match key.code {
            KeyCode::Enter => state.input_mode = InputMode::Normal,
            KeyCode::Char(c) => handle_char_input(c, state),
            KeyCode::Backspace => handle_backspace(state),
            KeyCode::Esc => state.input_mode = InputMode::Normal,
            _ => {}
        },
    }
    Ok(false)
}

fn handle_char_input(c: char, state: &mut AppState) {
    match state.input_mode {
        InputMode::Start1 => state.start1.push(c),
        InputMode::Start2 => state.start2.push(c),
        InputMode::RangeStart => state.range_start.push(c),
        InputMode::RangeEnd => state.range_end.push(c),
        InputMode::FilterValue => state.filter_value.push(c),
        _ => {}
    }
}

fn handle_backspace(state: &mut AppState) {
    match state.input_mode {
        InputMode::Start1 => {
            state.start1.pop();
        }
        InputMode::Start2 => {
            state.start2.pop();
        }
        InputMode::RangeStart => {
            state.range_start.pop();
        }
        InputMode::RangeEnd => {
            state.range_end.pop();
        }
        InputMode::FilterValue => {
            state.filter_value.pop();
        }
        _ => {}
    }
}
