use color_eyre::Result;
use ratatui::{
    crossterm::event::{
        self, Event, KeyCode, KeyEventKind, MouseButton, MouseEvent, MouseEventKind,
    },
    layout::Rect,
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
    /// Last computed area for messages (used for mouse click detection)
    messages_area: Option<Rect>,
    /// Last computed area for input (used for mouse click detection)
    input_area: Option<Rect>,
    /// Last mouse event captured (handled inside draw at widget level)
    last_mouse_event: Option<MouseEvent>,
    /// Vertical scroll offset for the messages list (index of the top-most message shown)
    messages_scroll: usize,
}

pub enum InputMode {
    Normal,
    Editing,
}

impl App {
    pub const fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            character_index: 0,
            messages_area: None,
            input_area: None,
            last_mouse_event: None,
            messages_scroll: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    /// Delete the character at the cursor (forward delete).
    fn delete_forward(&mut self) {
        let char_count = self.input.chars().count();
        // Only delete if the cursor is not at the end
        if self.character_index < char_count {
            let current_index = self.character_index;

            // Characters before the one to delete
            let before = self.input.chars().take(current_index);
            // Characters after the one to delete (skip the current char)
            let after = self.input.chars().skip(current_index + 1);

            // Rebuild string without the deleted character
            self.input = before.chain(after).collect();

            // Keep the cursor at the same logical position, clamp just in case
            self.character_index = self.clamp_cursor(self.character_index);
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
        // scroll to bottom when a new message is submitted
        let inner_height = self
            .messages_area
            .map(|a| a.height.saturating_sub(2) as usize)
            .unwrap_or(0);
        if inner_height > 0 && self.messages.len() > inner_height {
            self.messages_scroll = self.messages.len() - inner_height;
        } else {
            self.messages_scroll = 0;
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        // enable mouse capture so terminal delivers mouse events
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

        loop {
            // draw and capture the latest areas for hit-testing
            terminal.draw(|frame| self.draw(frame))?;

            // read an input event (keyboard or mouse)
            match event::read()? {
                Event::Key(key) => match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Up => {
                            // scroll up one line in the messages view
                            if let Some(area) = self.messages_area {
                                let inner = area.height.saturating_sub(2) as usize;
                                if inner > 0 && self.messages_scroll > 0 {
                                    self.messages_scroll = self.messages_scroll.saturating_sub(1);
                                }
                            }
                        }
                        KeyCode::Down => {
                            // scroll down one line in the messages view
                            if let Some(area) = self.messages_area {
                                let inner = area.height.saturating_sub(2) as usize;
                                if inner > 0 && self.messages.len() > inner {
                                    let max_start = self.messages.len() - inner;
                                    self.messages_scroll =
                                        (self.messages_scroll + 1).min(max_start);
                                }
                            }
                        }
                        KeyCode::Char('e') => {
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Tab => {
                            // Toggle to editing mode
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') | KeyCode::Esc => {
                            // disable mouse capture and raw mode before exiting
                            crossterm::execute!(
                                std::io::stdout(),
                                crossterm::event::DisableMouseCapture
                            )?;
                            crossterm::terminal::disable_raw_mode()?;
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Delete => self.delete_forward(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc | KeyCode::Tab => self.input_mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Editing => {}
                },
                Event::Mouse(me) => {
                    // store the mouse event and let draw() handle the widget-level logic
                    self.last_mouse_event = Some(me);
                }
                // Ignore other event types
                _ => {}
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(3),
        ]);
        let [help_area, messages_area, input_area] = vertical.areas(frame.area());

        // store areas for hit-testing by the event loop
        self.messages_area = Some(messages_area);
        self.input_area = Some(input_area);

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " or ".into(),
                    "Esc".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " or ".into(),
                    "Tab".bold(),
                    " to start editing.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " or ".into(),
                    "Tab".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        // Handle scroll events (mouse wheel) that were captured by the event loop
        if let Some(me) = &self.last_mouse_event {
            match &me.kind {
                MouseEventKind::ScrollUp | MouseEventKind::ScrollDown => {
                    let col = me.column as i32;
                    let row = me.row as i32;
                    if col >= messages_area.x as i32
                        && col < (messages_area.x + messages_area.width) as i32
                        && row >= messages_area.y as i32
                        && row < (messages_area.y + messages_area.height) as i32
                    {
                        let inner_height = messages_area.height.saturating_sub(2) as usize;
                        if inner_height > 0 && self.messages.len() > inner_height {
                            let max_start = self.messages.len() - inner_height;
                            match &me.kind {
                                MouseEventKind::ScrollUp => {
                                    self.messages_scroll = self.messages_scroll.saturating_sub(1);
                                }
                                MouseEventKind::ScrollDown => {
                                    self.messages_scroll =
                                        (self.messages_scroll + 1).min(max_start);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // clamp scroll to valid range and compute visible slice
        let inner_height = messages_area.height.saturating_sub(2) as usize;
        let start_idx = if inner_height == 0 || self.messages.len() <= inner_height {
            0usize
        } else {
            let max_start = self.messages.len() - inner_height;
            self.messages_scroll.min(max_start)
        };
        let visible_count = inner_height.min(self.messages.len());
        let visible_messages = if visible_count == 0 {
            Vec::new()
        } else {
            self.messages
                .iter()
                .enumerate()
                .skip(start_idx)
                .take(visible_count)
                .map(|(i, m)| {
                    let content = Line::from(Span::raw(format!("{i}: {m}")));
                    ListItem::new(content)
                })
                .collect()
        };
        let messages_block = Block::bordered().title("Messages");
        let messages_widget = List::new(visible_messages).block(messages_block.clone());
        frame.render_widget(messages_widget, messages_area);

        // Widget-level mouse handling for messages/input
        if let Some(me) = &self.last_mouse_event {
            match &me.kind {
                MouseEventKind::Down(btn) if *btn == MouseButton::Left => {
                    let col = me.column as i32;
                    let row = me.row as i32;
                    if col >= messages_area.x as i32
                        && col < (messages_area.x + messages_area.width) as i32
                        && row >= messages_area.y as i32
                        && row < (messages_area.y + messages_area.height) as i32
                    {
                        // Click inside messages: deselect input
                        self.input_mode = InputMode::Normal;
                    }
                }
                _ => {}
            }
        }

        let input_block = Block::bordered().title("Input");
        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(input_block.clone());
        frame.render_widget(input, input_area);

        // Widget-level mouse handling for input (position cursor and set editing)
        if let Some(me) = &self.last_mouse_event {
            match &me.kind {
                MouseEventKind::Down(btn) if *btn == MouseButton::Left => {
                    let col = me.column as i32;
                    let row = me.row as i32;
                    if col >= input_area.x as i32
                        && col < (input_area.x + input_area.width) as i32
                        && row >= input_area.y as i32
                        && row < (input_area.y + input_area.height) as i32
                    {
                        // set editing mode and position cursor
                        self.input_mode = InputMode::Editing;
                        let char_pos = (col - input_area.x as i32 - 1).max(0) as usize;
                        let clamped = char_pos.clamp(0, self.input.chars().count());
                        self.character_index = clamped;
                    }
                }
                _ => {}
            }
        }

        // clear the mouse event after widgets had a chance to handle it
        self.last_mouse_event = None;
        match self.input_mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            InputMode::Normal => {}

            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                input_area.x + self.character_index as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + 1,
            )),
        }
    }
}
