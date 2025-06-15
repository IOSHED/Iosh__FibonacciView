use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use num_bigint::BigInt;
use ratatui::layout::Constraint::{Fill, Length, Min};
use ratatui::layout::Layout;
use ratatui::prelude::{Span, Style};
use ratatui::style::Stylize;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};
use std::str::FromStr;

use fibo_calc::{FiboBuilder, FiboCalc};

pub struct TerminalApp {
    terminal: DefaultTerminal,
    state: AppState,
}

struct AppState {
    start1: String,
    start2: String,
    range_start: String,
    range_end: String,
    filter_value: String,

    filters: Vec<Filter>,

    // Output
    results: Vec<BigInt>,
    list_state: ListState,

    // UI state
    input_mode: InputMode,
    count_use: usize,
    error: Option<String>,
}

#[derive(PartialEq)]
enum InputMode {
    Start1,
    Start2,
    RangeStart,
    RangeEnd,
    FilterValue,
    Normal,
}

#[derive(Clone)]
struct Filter {
    filter_type: FilterType,
    value: BigInt,
}

#[derive(Clone)]
enum FilterType {
    Ge,
    Le,
}

impl TerminalApp {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            terminal: ratatui::init(),
            state: AppState {
                start1: "0".to_string(),
                start2: "1".to_string(),
                range_start: "0".to_string(),
                range_end: "100".to_string(),
                filter_value: "10".to_string(),
                filters: Vec::new(),
                results: Vec::new(),
                list_state,
                input_mode: InputMode::Normal,
                count_use: 0,
                error: None,
            },
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        loop {
            self.terminal
                .draw(|f| Self::draw(f, &mut self.state))
                .expect("failed to draw frame");

            if Self::handle_events(&mut self.state)? {
                break Ok(());
            }
        }
    }

    pub fn restore(self) {
        ratatui::restore();
    }

    fn handle_events(state: &mut AppState) -> std::io::Result<bool> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => Self::handle_keys(key, state),
            _ => Ok(false),
        }
    }

    fn handle_keys(key: KeyEvent, state: &mut AppState) -> std::io::Result<bool> {
        match state.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('1') => state.input_mode = InputMode::Start1,
                KeyCode::Char('2') => state.input_mode = InputMode::Start2,
                KeyCode::Char('s') => state.input_mode = InputMode::RangeStart,
                KeyCode::Char('e') => state.input_mode = InputMode::RangeEnd,
                KeyCode::Char('v') => state.input_mode = InputMode::FilterValue,
                KeyCode::Char('a') => Self::add_filter(state),
                KeyCode::Char('r') => Self::calculate(state),
                KeyCode::Char('c') => state.filters.clear(),
                KeyCode::Char('d') => Self::delete_filter(state),
                KeyCode::Up => Self::scroll_results(state, -1),
                KeyCode::Down => Self::scroll_results(state, 1),
                _ => {}
            },
            _ => match key.code {
                KeyCode::Enter => state.input_mode = InputMode::Normal,
                KeyCode::Char(c) => Self::handle_char_input(c, state),
                KeyCode::Backspace => Self::handle_backspace(state),
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

    fn add_filter(state: &mut AppState) {
        if let Ok(value) = BigInt::from_str(&state.filter_value) {
            state.filters.push(Filter {
                filter_type: FilterType::Ge,
                value: value.clone(),
            });
            state.filters.push(Filter {
                filter_type: FilterType::Le,
                value: value + BigInt::from(100),
            });
        }
    }

    fn delete_filter(state: &mut AppState) {
        if !state.filters.is_empty() {
            state.filters.pop();
        }
    }

    fn scroll_results(state: &mut AppState, direction: i32) {
        if state.results.is_empty() {
            return;
        }

        let selected = state.list_state.selected().unwrap_or(0);
        let new_index = match direction {
            1 => (selected + 1) % state.results.len(),
            -1 => (selected + state.results.len() - 1) % state.results.len(),

            _ => selected,
        };
        state.list_state.select(Some(new_index));
    }

    fn calculate(state: &mut AppState) {
        state.error = None;
        state.count_use += 1;

        // Parse inputs
        let start1 = match BigInt::from_str(&state.start1) {
            Ok(n) => n,
            Err(_) => {
                state.error = Some("Invalid start number 1".to_string());
                return;
            }
        };

        let start2 = match BigInt::from_str(&state.start2) {
            Ok(n) => n,
            Err(_) => {
                state.error = Some("Invalid start number 2".to_string());
                return;
            }
        };

        let range_start = match state.range_start.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                state.error = Some("Invalid range start".to_string());
                return;
            }
        };

        let range_end = match state.range_end.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                state.error = Some("Invalid range end".to_string());
                return;
            }
        };

        if range_end <= range_start {
            state.error = Some("Range end must be > start".to_string());
            return;
        }

        // Build Fibonacci calculator
        let mut builder = FiboBuilder::default();
        builder
            .set_start_nums(Some((start1, start2)))
            .set_range_by_id(Some(range_start..range_end));

        // Add filters using closures
        for filter in &state.filters {
            let value = filter.value.clone();
            match filter.filter_type {
                FilterType::Ge => builder.add_filter(move |num| num >= &value),
                FilterType::Le => builder.add_filter(move |num| num <= &value),
            };
        }

        // Calculate results
        let fibo_calc = FiboCalc::new(builder);
        state.results = fibo_calc.calc();
        state.list_state.select(Some(0));
    }

    fn draw(frame: &mut Frame, state: &mut AppState) {
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([Fill(1); 2]);
        let [left_area, right_area] = horizontal.areas(main_area);

        // Title
        let mut main_title = Line::from("FIBONACCI").centered().cyan();
        main_title.push_span(Span::raw(" VIEW").style(Style::new().light_red()));

        // Status
        let status_text = format!(
            " Uses: {} | Elements: {} | Filters: {} ",
            state.count_use,
            state.results.len(),
            state.filters.len()
        );
        let status_title = Line::from(status_text);

        // Create blocks without moving
        let input_block = Block::bordered().title(" Input ").light_red();
        let output_block = Block::bordered().title(" Output ").cyan();

        // Draw title and status
        frame.render_widget(Block::bordered().title(main_title).dark_gray(), title_area);
        frame.render_widget(
            Block::bordered().title(status_title).dark_gray(),
            status_area,
        );

        // Render widgets by reference
        frame.render_widget(&input_block, left_area);
        frame.render_widget(&output_block, right_area);

        // Draw input panel content
        let inner_left = input_block.inner(left_area);
        frame.render_widget(Self::render_input_panel(state), inner_left);

        // Draw output panel content
        let inner_right = output_block.inner(right_area);
        frame.render_widget(Self::render_output_panel(state), inner_right);
    }

    fn render_input_panel(state: &AppState) -> Paragraph {
        let mut lines = vec![
            Line::from(format!("Start 1 [1]: {}", state.start1)),
            Line::from(format!("Start 2 [2]: {}", state.start2)),
            Line::from(format!("Range Start [s]: {}", state.range_start)),
            Line::from(format!("Range End [e]: {}", state.range_end)),
            Line::from(format!("Filter Value [v]: {}", state.filter_value)),
            Line::from(""),
            Line::from("Filters:"),
        ];

        for filter in &state.filters {
            let symbol = match filter.filter_type {
                FilterType::Ge => "≥",
                FilterType::Le => "≤",
            };
            lines.push(Line::from(format!("{} {}", symbol, filter.value)));
        }

        lines.extend([
            Line::from(""),
            Line::from("[a]dd filters  [d]elete filter"),
            Line::from("[r]ecalculate  [c]lear filters"),
            Line::from("Arrow keys: Navigate results"),
            Line::from("Press 1,2,s,e,v to edit fields"),
        ]);

        // Highlight active field
        if state.input_mode != InputMode::Normal {
            let field_index = match state.input_mode {
                InputMode::Start1 => 0,
                InputMode::Start2 => 1,
                InputMode::RangeStart => 2,
                InputMode::RangeEnd => 3,
                InputMode::FilterValue => 4,
                _ => 0,
            };

            if let Some(line) = lines.get_mut(field_index) {
                *line = line.clone().style(Style::new().yellow());
            }
        }

        // Show error if exists
        let mut text = Text::from(lines);
        if let Some(err) = &state.error {
            text.lines
                .push(Line::from(err.clone()).style(Style::new().red()));
        }

        Paragraph::new(text).wrap(Wrap { trim: true })
    }

    fn render_output_panel(state: &mut AppState) -> List {
        let items: Vec<_> = state
            .results
            .iter()
            .enumerate()
            .map(|(i, num)| ListItem::new(format!("{:4}: {}", i, num)))
            .collect();

        List::new(items)
            .highlight_style(Style::new().yellow())
            .block(Block::default())
    }
}
