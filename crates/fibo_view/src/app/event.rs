use crate::app::state::InputMode;
use crate::app::{AppState, FilterType};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::{Error, Result};

pub struct EventHandler {
    state: AppState,
}

impl EventHandler {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub fn handle_events(&mut self) -> Result<bool> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_event(key),
            _ => Ok(false),
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<bool> {
        match self.state.input_mode {
            InputMode::Normal => self.handle_normal_mode(key),
            _ => self.handle_input_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char(c) => self.handle_normal_char(c),
            KeyCode::Up => {
                self.state.scroll_results(-1);
                Ok(false)
            }
            KeyCode::Down => {
                self.state.scroll_results(1);
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_normal_char(&mut self, c: char) -> Result<bool> {
        match c {
            'q' => Ok(true),
            '1' => {
                self.state.input_mode = InputMode::Start1;
                self.state.input.start1.clear();
                Ok(false)
            },
            '2' =>{
                self.state.input_mode = InputMode::Start2;
                self.state.input.start2.clear();
                Ok(false)
            },
            's' => {
                self.state.input_mode = InputMode::RangeStart;
                self.state.input.range_start.clear();
                Ok(false)
            },
            'e' =>{
                self.state.input_mode = InputMode::RangeEnd;
                self.state.input.range_end.clear();
                Ok(false)
            },
            'v' => {
                self.state.input_mode = InputMode::FilterValue;
                self.state.input.filter_value.clear();
                Ok(false)
            },
            'g' => {
                self.state.filters.filter_type = FilterType::Ge;
                Ok(false)
            }
            'l' => {
                self.state.filters.filter_type = FilterType::Le;
                Ok(false)
            }
            'a' => {
                self.state.add_filter().map_err(|e| Error::new(std::io::ErrorKind::Other, e))?;
                Ok(false)
            }
            'r' => {
                self.state.calculate().map_err(|e| Error::new(std::io::ErrorKind::Other, e))?;
                Ok(false)
            }
            'c' => {
                self.state.clear_filters();
                Ok(false)
            }
            'd' => {
                self.state.delete_filter();
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_input_mode(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Enter | KeyCode::Esc => {
                self.state.input_mode = InputMode::Normal;
                Ok(false)
            }
            KeyCode::Char(c) => {
                self.handle_input_char(c);
                Ok(false)
            }
            KeyCode::Backspace => {
                self.handle_backspace();
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_input_char(&mut self, c: char) {
        let field = self.get_current_input_field();
        if let Some(field) = field {
            field.push(c);
        }
    }

    fn handle_backspace(&mut self) {
        let field = self.get_current_input_field();
        if let Some(field) = field {
            field.pop();
        }
    }

    fn get_current_input_field(&mut self) -> Option<&mut String> {
        match self.state.input_mode {
            InputMode::Start1 => Some(&mut self.state.input.start1),
            InputMode::Start2 => Some(&mut self.state.input.start2),
            InputMode::RangeStart => Some(&mut self.state.input.range_start),
            InputMode::RangeEnd => Some(&mut self.state.input.range_end),
            InputMode::FilterValue => Some(&mut self.state.input.filter_value),
            InputMode::Normal => None,
        }
    }
}

pub fn handle_events(state: &mut AppState) -> Result<bool> {
    let mut handler = EventHandler::new(std::mem::replace(state, AppState::default()));
    let result = handler.handle_events();
    *state = handler.state;
    result
}
