use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame, Terminal,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io;

use crate::auth::UserContext;
use crate::db::repositories::SessionRepository;
use crate::models::Session;
use crate::ui::navigation::Screen;

pub struct App {
    pub user_context: UserContext,
    pub should_quit: bool,
    pub current_screen: Screen,
    pub sessions: Vec<Session>,
    pub selected_index: usize,
    pub db_path: String,
}

impl App {
    pub fn new(user_context: UserContext, db_path: String) -> Self {
        Self {
            user_context,
            should_quit: false,
            current_screen: Screen::Home,
            sessions: Vec::new(),
            selected_index: 0,
            db_path,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                // Only handle KeyPress events (ignore KeyRelease and KeyRepeat)
                if key.kind == KeyEventKind::Press {
                    self.handle_key_event(key);
                }
            }

            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                // If on home screen, quit; otherwise go back to home
                if self.current_screen == Screen::Home {
                    self.should_quit = true;
                } else {
                    self.current_screen = Screen::Home;
                    self.selected_index = 0;
                }
            }
            KeyCode::Char('1') => {
                self.current_screen = Screen::Home;
                self.selected_index = 0;
            }
            KeyCode::Char('2') => {
                self.load_sessions();
                self.current_screen = Screen::SessionList;
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                // Create session (coach only)
                if self.user_context.is_coach() && self.current_screen == Screen::SessionList {
                    self.current_screen = Screen::SessionCreate;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.sessions.is_empty() && self.selected_index < self.sessions.len() - 1 {
                    self.selected_index += 1;
                }
            }
            KeyCode::Enter => {
                if self.current_screen == Screen::SessionList && !self.sessions.is_empty() {
                    let session_id = self.sessions[self.selected_index].id;
                    self.current_screen = Screen::SessionDetail(session_id);
                }
            }
            _ => {}
        }
    }

    fn load_sessions(&mut self) {
        if let Ok(conn) = crate::db::establish_connection(&self.db_path) {
            self.sessions = if self.user_context.is_coach() {
                SessionRepository::find_by_coach(&conn, self.user_context.user.id)
                    .unwrap_or_default()
            } else {
                // For players, we'll later filter to subscribed sessions
                SessionRepository::find_all(&conn).unwrap_or_default()
            };
            self.selected_index = 0;
        }
    }

    fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main content
                Constraint::Length(3), // Footer
            ])
            .split(frame.size());

        // Header
        self.render_header(frame, chunks[0]);

        // Main content - render based on current screen
        match &self.current_screen {
            Screen::Home => self.render_home(frame, chunks[1]),
            Screen::SessionList => self.render_session_list(frame, chunks[1]),
            Screen::SessionDetail(id) => self.render_session_detail(frame, chunks[1], *id),
            Screen::SessionCreate => self.render_session_create(frame, chunks[1]),
            _ => self.render_home(frame, chunks[1]),
        }

        // Footer
        self.render_footer(frame, chunks[2]);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header = Paragraph::new(vec![Line::from(vec![
            Span::styled(
                "Tennis Training App",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            Span::styled(
                format!("User: {}", self.user_context.user.display_name),
                Style::default().fg(Color::Green),
            ),
            Span::raw(" | "),
            Span::styled(
                format!("Role: {}", self.user_context.user.role.as_str()),
                Style::default().fg(Color::Yellow),
            ),
        ])])
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

        frame.render_widget(header, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let footer_text = match &self.current_screen {
            Screen::Home => "Press [1] Home | [2] Sessions | [q] Quit",
            Screen::SessionList => {
                if self.user_context.is_coach() {
                    "↑↓ Navigate | [Enter] View | [c] Create | [Esc] Back | [q] Quit"
                } else {
                    "↑↓ Navigate | [Enter] View | [Esc] Back | [q] Quit"
                }
            }
            Screen::SessionDetail(_) => "[Esc] Back | [q] Quit",
            Screen::SessionCreate => "[Esc] Cancel | [q] Quit",
            _ => "[Esc] Back | [q] Quit",
        };

        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(footer, area);
    }

    fn render_home(&self, frame: &mut Frame, area: Rect) {
        let welcome_text = if self.user_context.is_coach() {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "Welcome, Coach!",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from("You can manage training sessions and monitor player progress."),
                Line::from(""),
                Line::from("Navigation:"),
                Line::from("  [1] Home"),
                Line::from("  [2] Manage Sessions"),
                Line::from(""),
                Line::from("Press the number key to navigate."),
            ]
        } else {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    format!("Welcome, {}!", self.user_context.user.display_name),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(format!(
                    "Skill Level: {}",
                    self.user_context
                        .user
                        .skill_level
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or("Not set")
                )),
                Line::from(""),
                Line::from("Navigation:"),
                Line::from("  [1] Home"),
                Line::from("  [2] My Sessions"),
                Line::from(""),
                Line::from("Press the number key to navigate."),
            ]
        };

        let main_content = Paragraph::new(welcome_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Home")
                    .style(Style::default()),
            )
            .alignment(Alignment::Center);

        frame.render_widget(main_content, area);
    }

    fn render_session_list(&self, frame: &mut Frame, area: Rect) {
        let title = if self.user_context.is_coach() {
            "Manage Sessions"
        } else {
            "My Sessions"
        };

        if self.sessions.is_empty() {
            let empty_text = vec![
                Line::from(""),
                Line::from(""),
                Line::from(Span::styled(
                    "No sessions found",
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(""),
                Line::from(if self.user_context.is_coach() {
                    "Press [c] to create your first session"
                } else {
                    "No sessions assigned yet"
                }),
            ];

            let content = Paragraph::new(empty_text)
                .block(Block::default().borders(Borders::ALL).title(title))
                .alignment(Alignment::Center);

            frame.render_widget(content, area);
            return;
        }

        let items: Vec<ListItem> = self
            .sessions
            .iter()
            .enumerate()
            .map(|(i, session)| {
                let date_str = session
                    .scheduled_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "No date".to_string());

                let time_str = session
                    .scheduled_time
                    .map(|t| t.format("%H:%M").to_string())
                    .unwrap_or_else(|| "".to_string());

                let duration_str = session
                    .duration_minutes
                    .map(|d| format!(" ({}min)", d))
                    .unwrap_or_default();

                let line = if i == self.selected_index {
                    Line::from(vec![
                        Span::styled(
                            "► ",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            &session.title,
                            Style::default()
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(format!(" - {} {}{}", date_str, time_str, duration_str)),
                    ])
                } else {
                    Line::from(format!(
                        "  {} - {} {}{}",
                        session.title, date_str, time_str, duration_str
                    ))
                };

                ListItem::new(line)
            })
            .collect();

        let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));

        frame.render_widget(list, area);
    }

    fn render_session_detail(&self, frame: &mut Frame, area: Rect, session_id: i64) {
        // Find the session
        let session = self.sessions.iter().find(|s| s.id == session_id);

        let content = if let Some(session) = session {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    &session.title,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(format!(
                    "Date: {}",
                    session
                        .scheduled_date
                        .map(|d| d.format("%Y-%m-%d").to_string())
                        .unwrap_or_else(|| "Not scheduled".to_string())
                )),
                Line::from(format!(
                    "Time: {}",
                    session
                        .scheduled_time
                        .map(|t| t.format("%H:%M").to_string())
                        .unwrap_or_else(|| "Not set".to_string())
                )),
                Line::from(format!(
                    "Duration: {} minutes",
                    session.duration_minutes.unwrap_or(0)
                )),
                Line::from(format!(
                    "Skill Level: {}",
                    session
                        .skill_level
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or("Any")
                )),
                Line::from(""),
                Line::from("Description:"),
                Line::from(
                    session
                        .description
                        .as_ref()
                        .map(|s| s.as_str())
                        .unwrap_or("No description"),
                ),
                Line::from(""),
                Line::from(Span::styled(
                    "Training content will be displayed here in future updates",
                    Style::default().fg(Color::DarkGray),
                )),
            ]
        } else {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "Session not found",
                    Style::default().fg(Color::Red),
                )),
            ]
        };

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Session Details"),
            )
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, area);
    }

    fn render_session_create(&self, frame: &mut Frame, area: Rect) {
        let content = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Create Session",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("Session creation form will be implemented in the next phase."),
            Line::from(""),
            Line::from("For now, you can add sessions directly to the database:"),
            Line::from(""),
            Line::from("  sqlite3 data/tennis.db"),
            Line::from(
                "  INSERT INTO sessions (title, description, created_by, created_at, updated_at)",
            ),
            Line::from(
                "  VALUES ('Test Session', 'A test session', 1, datetime('now'), datetime('now'));",
            ),
        ];

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Create Session"),
            )
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, area);
    }
}
