use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame, Terminal,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use rusqlite::Connection;
use std::io;

use crate::auth::UserContext;
use crate::db::repositories::session_repo::SessionRepository;
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
    pub fn new(user_context: UserContext) -> Self {
        Self {
            user_context,
            should_quit: false,
            current_screen: Screen::Home,
            sessions: Vec::new(),
            selected_index: 0,
            db_path: db_path.to_string(),
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
            KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main Content
                Constraint::Length(3), // Footer
            ])
            .split(frame.size());

        // Header
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

        frame.render_widget(header, chunks[0]);

        // Main content
        let welcome_text = if self.user_context.is_coach() {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "Welcome Coach!",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from("You can manage training sessions and monitor players' progress."),
                Line::from(""),
                Line::from("Features coming soon:"),
                Line::from("  • Create and edit sessions"),
                Line::from("  • View all players"),
                Line::from("  • Track player progress"),
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
                Line::from("You can view your training sessions and progress."),
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
                Line::from("Features coming soon:"),
                Line::from("  • View your training sessions"),
                Line::from("  • Browse available sessions"),
                Line::from("  • Track your progress"),
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

        frame.render_widget(main_content, chunks[1]);

        // Footer
        let footer = Paragraph::new(Line::from(vec![
            Span::raw("Press "),
            Span::styled(
                "q",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" or "),
            Span::styled(
                "Esc",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" to quit"),
        ]))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

        frame.render_widget(footer, chunks[2]);
    }
}
