use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::str::FromStr;

use crate::auth::UserContext;
use crate::db::repositories::{SessionRepository, SubscriptionRepository, TrainingContentRepository};
use crate::models::{SessionWithSubscription, TrainingContent};
use crate::ui::navigation::Screen;
use crate::ui::session_filter::SessionFilter;
use crate::ui::session_form::SessionForm;
use crate::ui::session_edit_form::SessionEditForm;
use crate::ui::text_editor::TextEditor;
use crate::ui::help::HelpScreen;
use crate::ui::training_content_form::TrainingContentForm;

pub struct App {
    pub user_context: UserContext,
    pub should_quit: bool,
    pub current_screen: Screen,
    pub sessions: Vec<SessionWithSubscription>,
    pub selected_index: usize,
    pub db_path: String,
    pub session_filter: SessionFilter,
    pub message: Option<String>,
    pub session_form: SessionForm,
    pub session_edit_form: Option<SessionEditForm>,
    pub delete_confirmation: bool,
    pub training_content: Vec<TrainingContent>,
    pub training_content_form: TrainingContentForm,
    pub training_content_selected_index: usize,
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
            session_filter: SessionFilter::MySubscriptions,
            message: None,
            session_form: SessionForm::new(),
            session_edit_form: None,
            delete_confirmation: false,
            training_content: Vec::new(),
            training_content_form: TrainingContentForm::new(),
            training_content_selected_index: 0,
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
        // Handle form input separately
        if self.current_screen == Screen::SessionCreate || matches!(self.current_screen, Screen::SessionEdit(_)) {
            self.handle_form_key_event(key);
            return;
        }

        // Handle training content form input
        if matches!(self.current_screen, Screen::TrainingContentCreate(_) | Screen::TrainingContentEdit(_)) {
            self.handle_training_content_form_key_event(key);
            return;
        }

        // Handle delete confirmation
        if matches!(self.current_screen, Screen::SessionDelete(_) | Screen::TrainingContentDelete(_)) {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if let Screen::SessionDelete(session_id) = self.current_screen {
                        self.delete_session(session_id);
                    } else if let Screen::TrainingContentDelete(content_id) = self.current_screen {
                        self.delete_training_content(content_id);
                    }
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.current_screen = Screen::SessionList;
                    self.load_sessions();
                }
                _ => {}
            }
            return;
        }

        // Clear message on any key press
        self.message = None;

        match key.code {
            KeyCode::Char('?') => {
                // Show help screen
                self.current_screen = Screen::Help;
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                // Quit only from home screen, or close help
                if self.current_screen == Screen::Help {
                    self.current_screen = Screen::Home;
                } else if self.current_screen == Screen::Home {
                    self.should_quit = true;
                } else {
                    self.current_screen = Screen::Home;
                    self.selected_index = 0;
                }
            }
            KeyCode::Esc => {
                // Go back/cancel
                if self.current_screen == Screen::Help {
                    self.current_screen = Screen::Home;
                } else {
                    self.current_screen = Screen::Home;
                    self.selected_index = 0;
                    self.training_content.clear();
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
                    self.session_form = SessionForm::new();
                    self.current_screen = Screen::SessionCreate;
                }
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                // Toggle filter (player only, on session list)
                if self.user_context.is_player() && self.current_screen == Screen::SessionList {
                    self.session_filter = self.session_filter.toggle();
                    self.load_sessions();
                }
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                // Subscribe/Unsubscribe (player only, on session list)
                if self.user_context.is_player() && self.current_screen == Screen::SessionList {
                    self.toggle_subscription();
                }
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                // Mark complete (player only, on session detail)
                if self.user_context.is_player() {
                    if let Screen::SessionDetail(session_id) = self.current_screen {
                        self.mark_session_complete(session_id);
                    }
                }
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                // Training content management (coach only, on session detail)
                if self.user_context.is_coach() {
                    if let Screen::SessionDetail(session_id) = self.current_screen {
                        self.training_content_form = TrainingContentForm::new();
                        self.current_screen = Screen::TrainingContentCreate(session_id);
                    }
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
                    let session_id = self.sessions[self.selected_index].session.id;
                    self.current_screen = Screen::SessionDetail(session_id);
                    
                    // Load training content for this session
                    if let Ok(conn) = crate::db::connection::establish_connection(&self.db_path) {
                        if let Ok(content) = TrainingContentRepository::find_by_session(&conn, session_id) {
                            self.training_content = content;
                        }
                    }
                }
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                // Edit session (coach only, on session list)
                if self.user_context.is_coach() && self.current_screen == Screen::SessionList && !self.sessions.is_empty() {
                    let session_id = self.sessions[self.selected_index].session.id;
                    if let Some(session_with_sub) = self.sessions.iter().find(|s| s.session.id == session_id) {
                        self.session_edit_form = Some(SessionEditForm::from_session(&session_with_sub.session));
                        self.current_screen = Screen::SessionEdit(session_id);
                    }
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                // Delete session (coach only, on session list)
                if self.user_context.is_coach() && self.current_screen == Screen::SessionList && !self.sessions.is_empty() {
                    let session_id = self.sessions[self.selected_index].session.id;
                    self.current_screen = Screen::SessionDelete(session_id);
                }
            }
            _ => {}
        }
    }

    fn handle_form_key_event(&mut self, key: KeyEvent) {
        let is_create = matches!(self.current_screen, Screen::SessionCreate);
        let is_edit = matches!(self.current_screen, Screen::SessionEdit(_));

        match key.code {
            KeyCode::Tab => {
                if is_create {
                    self.session_form.next_field();
                } else if is_edit {
                    if let Some(form) = &mut self.session_edit_form {
                        form.next_field();
                    }
                }
            }
            KeyCode::BackTab => {
                if is_create {
                    self.session_form.prev_field();
                } else if is_edit {
                    if let Some(form) = &mut self.session_edit_form {
                        form.prev_field();
                    }
                }
            }
            KeyCode::Char(c) => {
                if is_create {
                    self.session_form.add_char(c);
                } else if is_edit {
                    if let Some(form) = &mut self.session_edit_form {
                        form.add_char(c);
                    }
                }
            }
            KeyCode::Backspace => {
                if is_create {
                    self.session_form.backspace();
                } else if is_edit {
                    if let Some(form) = &mut self.session_edit_form {
                        form.backspace();
                    }
                }
            }
            KeyCode::Left => {
                // For skill level navigation
                if is_create {
                    if self.session_form.focus_field == crate::ui::session_form::FormField::SkillLevel {
                        self.session_form.focus_field = crate::ui::session_form::FormField::DurationMinutes;
                    }
                } else if is_edit {
                    if let Some(form) = &mut self.session_edit_form {
                        if form.focus_field == crate::ui::session_edit_form::FormField::SkillLevel {
                            form.focus_field = crate::ui::session_edit_form::FormField::DurationMinutes;
                        }
                    }
                }
            }
            KeyCode::Right => {
                // For skill level navigation
                if is_create {
                    if self.session_form.focus_field == crate::ui::session_form::FormField::SkillLevel {
                        self.session_form.cycle_skill_level_forward();
                    }
                } else if is_edit {
                    if let Some(form) = &mut self.session_edit_form {
                        if form.focus_field == crate::ui::session_edit_form::FormField::SkillLevel {
                            form.cycle_skill_level_forward();
                        }
                    }
                }
            }
            KeyCode::Up => {
                if is_create {
                    self.session_form.prev_field();
                } else if is_edit {
                    if let Some(form) = &mut self.session_edit_form {
                        form.prev_field();
                    }
                }
            }
            KeyCode::Down => {
                if is_create {
                    self.session_form.next_field();
                } else if is_edit {
                    if let Some(form) = &mut self.session_edit_form {
                        form.next_field();
                    }
                }
            }
            KeyCode::Enter => {
                // Validate and save
                if is_create {
                    match self.session_form.validate() {
                        Ok(()) => {
                            self.save_session();
                        }
                        Err(e) => {
                            self.message = Some(format!("Error: {}", e));
                        }
                    }
                } else if is_edit {
                    if let Some(form) = &mut self.session_edit_form {
                        match form.validate() {
                            Ok(()) => {
                                if let Screen::SessionEdit(session_id) = self.current_screen {
                                    self.update_session(session_id);
                                }
                            }
                            Err(e) => {
                                self.message = Some(format!("Error: {}", e));
                            }
                        }
                    }
                }
            }
            KeyCode::Esc => {
                self.current_screen = Screen::SessionList;
                self.load_sessions();
            }
            _ => {}
        }
    }

    fn handle_training_content_form_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => {
                self.training_content_form.next_field();
            }
            KeyCode::BackTab => {
                self.training_content_form.prev_field();
            }
            KeyCode::Char(c) => {
                self.training_content_form.add_char(c);
            }
            KeyCode::Backspace => {
                self.training_content_form.backspace();
            }
            KeyCode::Left | KeyCode::Up => {
                // For content type cycling backwards
                if self.training_content_form.focus_field == crate::ui::training_content_form::FormField::ContentType {
                    self.training_content_form.focus_field = crate::ui::training_content_form::FormField::DurationMinutes;
                } else {
                    self.training_content_form.prev_field();
                }
            }
            KeyCode::Right | KeyCode::Down => {
                // For content type cycling forward
                if self.training_content_form.focus_field == crate::ui::training_content_form::FormField::ContentType {
                    self.training_content_form.cycle_content_type_forward();
                } else {
                    self.training_content_form.next_field();
                }
            }
            KeyCode::Enter => {
                // Validate and save training content
                match self.training_content_form.validate() {
                    Ok(()) => {
                        if let Screen::TrainingContentCreate(session_id) = self.current_screen {
                            self.save_training_content(session_id);
                        } else if let Screen::TrainingContentEdit(content_id) = self.current_screen {
                            self.update_training_content(content_id);
                        }
                    }
                    Err(err) => {
                        self.message = Some(format!("Validation error: {}", err));
                    }
                }
            }
            KeyCode::Esc => {
                self.current_screen = Screen::SessionList;
                self.load_sessions();
            }
            _ => {}
        }
    }

    fn save_session(&mut self) {
        if let Ok(conn) = crate::db::establish_connection(&self.db_path) {
            let (title, description, date, time, duration, skill_level_str) = self.session_form.as_db_values();
            
            // Parse skill level
            let skill_level = crate::models::SkillLevel::from_str(&skill_level_str);
            
            // Parse dates/times
            let date_parsed = date.as_ref().and_then(|d| {
                chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()
            });
            let time_parsed = time.as_ref().and_then(|t| {
                chrono::NaiveTime::parse_from_str(t, "%H:%M").ok()
            });
            
            match SessionRepository::create(
                &conn,
                &title,
                if description.is_empty() { None } else { Some(description.as_str()) },
                date_parsed,
                time_parsed,
                duration,
                skill_level.as_ref(),
                self.user_context.user.id,
            ) {
                Ok(_) => {
                    self.message = Some("Session created successfully!".to_string());
                    self.current_screen = Screen::SessionList;
                    self.load_sessions();
                }
                Err(e) => {
                    self.message = Some(format!("Error saving session: {:?}", e));
                }
            }
        } else {
            self.message = Some("Error connecting to database".to_string());
        }
    }

    fn update_session(&mut self, session_id: i64) {
        if let Ok(conn) = crate::db::establish_connection(&self.db_path) {
            if let Some(form) = &self.session_edit_form {
                let (title, description, date, time, duration, skill_level_str) = form.as_db_values();
                
                // Parse skill level
                let skill_level = crate::models::SkillLevel::from_str(&skill_level_str);
                
                // Parse dates/times
                let date_parsed = date.as_ref().and_then(|d| {
                    chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()
                });
                let time_parsed = time.as_ref().and_then(|t| {
                    chrono::NaiveTime::parse_from_str(t, "%H:%M").ok()
                });
                
                match SessionRepository::update(
                    &conn,
                    session_id,
                    &title,
                    if description.is_empty() { None } else { Some(description.as_str()) },
                    date_parsed,
                    time_parsed,
                    duration,
                    skill_level.as_ref(),
                ) {
                    Ok(_) => {
                        self.message = Some("Session updated successfully!".to_string());
                        self.current_screen = Screen::SessionList;
                        self.session_edit_form = None;
                        self.load_sessions();
                    }
                    Err(e) => {
                        self.message = Some(format!("Error updating session: {:?}", e));
                    }
                }
            }
        } else {
            self.message = Some("Error connecting to database".to_string());
        }
    }

    fn delete_session(&mut self, session_id: i64) {
        if let Ok(conn) = crate::db::establish_connection(&self.db_path) {
            match SessionRepository::delete(&conn, session_id) {
                Ok(_) => {
                    self.message = Some("Session deleted successfully!".to_string());
                    self.current_screen = Screen::SessionList;
                    self.load_sessions();
                }
                Err(e) => {
                    self.message = Some(format!("Error deleting session: {:?}", e));
                }
            }
        } else {
            self.message = Some("Error connecting to database".to_string());
        }
    }

    fn save_training_content(&mut self, session_id: i64) {
        if let Ok(conn) = crate::db::establish_connection(&self.db_path) {
            let (title, description, duration, content_type_str) = self.training_content_form.as_db_values();
            
            // Parse content type
            let content_type = crate::models::ContentType::from_str(&content_type_str)
                .unwrap_or(crate::models::ContentType::Drill);
            
            // Empty description means None
            let desc = if description.is_empty() { None } else { Some(description.as_str()) };
            
            // Calculate order index (next available)
            let order_index = (self.training_content.len() as i32) + 1;
            
            match TrainingContentRepository::create(
                &conn,
                session_id,
                &content_type,
                &title,
                desc,
                duration,
                order_index,
            ) {
                Ok(_) => {
                    self.message = Some("Training content created successfully!".to_string());
                    self.current_screen = Screen::SessionDetail(session_id);
                    // Reload training content
                    if let Ok(content) = TrainingContentRepository::find_by_session(&conn, session_id) {
                        self.training_content = content;
                    }
                }
                Err(e) => {
                    self.message = Some(format!("Error creating content: {:?}", e));
                }
            }
        } else {
            self.message = Some("Error connecting to database".to_string());
        }
    }

    fn update_training_content(&mut self, content_id: i64) {
        if let Ok(conn) = crate::db::establish_connection(&self.db_path) {
            let (title, description, duration, content_type_str) = self.training_content_form.as_db_values();
            
            // Parse content type
            let content_type = crate::models::ContentType::from_str(&content_type_str)
                .unwrap_or(crate::models::ContentType::Drill);
            
            // Empty description means None
            let desc = if description.is_empty() { None } else { Some(description.as_str()) };
            
            // Find the current content to get session_id and order
            if let Some(current) = self.training_content.iter().find(|c| c.id == content_id) {
                let session_id = current.session_id;
                
                match TrainingContentRepository::update(
                    &conn,
                    content_id,
                    &content_type,
                    &title,
                    desc,
                    duration,
                    current.order_index,
                ) {
                    Ok(_) => {
                        self.message = Some("Training content updated successfully!".to_string());
                        self.current_screen = Screen::SessionDetail(session_id);
                        // Reload training content
                        if let Ok(content) = TrainingContentRepository::find_by_session(&conn, session_id) {
                            self.training_content = content;
                        }
                    }
                    Err(e) => {
                        self.message = Some(format!("Error updating content: {:?}", e));
                    }
                }
            }
        } else {
            self.message = Some("Error connecting to database".to_string());
        }
    }

    fn delete_training_content(&mut self, content_id: i64) {
        if let Ok(conn) = crate::db::establish_connection(&self.db_path) {
            // Find the session_id for this content
            if let Some(current) = self.training_content.iter().find(|c| c.id == content_id) {
                let session_id = current.session_id;
                
                match TrainingContentRepository::delete(&conn, content_id) {
                    Ok(_) => {
                        self.message = Some("Training content deleted successfully!".to_string());
                        self.current_screen = Screen::SessionDetail(session_id);
                        // Reload training content
                        if let Ok(content) = TrainingContentRepository::find_by_session(&conn, session_id) {
                            self.training_content = content;
                        }
                    }
                    Err(e) => {
                        self.message = Some(format!("Error deleting content: {:?}", e));
                    }
                }
            }
        } else {
            self.message = Some("Error connecting to database".to_string());
        }
    }

    fn load_sessions(&mut self) {
        if let Ok(conn) = crate::db::establish_connection(&self.db_path) {
            if self.user_context.is_coach() {
                // Coach sees all their created sessions
                let sessions = SessionRepository::find_by_coach(&conn, self.user_context.user.id)
                    .unwrap_or_default();
                
                self.sessions = sessions
                    .into_iter()
                    .map(|s| SessionWithSubscription::new(s, None))
                    .collect();
            } else {
                // Player sees sessions based on filter
                let all_sessions = SessionRepository::find_all(&conn).unwrap_or_default();
                let user_subscriptions = SubscriptionRepository::find_by_user(&conn, self.user_context.user.id)
                    .unwrap_or_default();

                self.sessions = all_sessions
                    .into_iter()
                    .map(|session| {
                        let subscription = user_subscriptions
                            .iter()
                            .find(|sub| sub.session_id == session.id)
                            .cloned();
                        SessionWithSubscription::new(session, subscription)
                    })
                    .filter(|sws| {
                        match self.session_filter {
                            SessionFilter::MySubscriptions => sws.is_subscribed(),
                            SessionFilter::AllAvailable => true,
                        }
                    })
                    .collect();
            }
            self.selected_index = 0;
        }
    }

    fn toggle_subscription(&mut self) {
        if self.sessions.is_empty() {
            return;
        }

        let session_with_sub = &self.sessions[self.selected_index];
        let session_id = session_with_sub.session.id;

        if let Ok(conn) = crate::db::establish_connection(&self.db_path) {
            if session_with_sub.is_subscribed() {
                // Unsubscribe
                if let Err(e) = SubscriptionRepository::delete_by_user_and_session(
                    &conn,
                    self.user_context.user.id,
                    session_id,
                ) {
                    self.message = Some(format!("Error unsubscribing: {}", e));
                } else {
                    self.message = Some("Unsubscribed successfully".to_string());
                    self.load_sessions();
                }
            } else {
                // Subscribe
                if let Err(e) = SubscriptionRepository::create(
                    &conn,
                    self.user_context.user.id,
                    session_id,
                ) {
                    self.message = Some(format!("Error subscribing: {}", e));
                } else {
                    self.message = Some("Subscribed successfully".to_string());
                    self.load_sessions();
                }
            }
        }
    }

    fn mark_session_complete(&mut self, session_id: i64) {
        if let Ok(conn) = crate::db::establish_connection(&self.db_path) {
            // Find the subscription
            if let Ok(Some(subscription)) = SubscriptionRepository::find_by_user_and_session(
                &conn,
                self.user_context.user.id,
                session_id,
            ) {
                if subscription.completed_at.is_some() {
                    self.message = Some("Session already marked as complete".to_string());
                } else {
                    if let Err(e) = SubscriptionRepository::mark_completed(&conn, subscription.id) {
                        self.message = Some(format!("Error marking complete: {}", e));
                    } else {
                        self.message = Some("Session marked as complete!".to_string());
                        self.load_sessions();
                    }
                }
            } else {
                self.message = Some("You must subscribe to this session first".to_string());
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),    // Header
                Constraint::Length(1),    // Message bar
                Constraint::Min(0),       // Main content
                Constraint::Length(3),    // Footer
            ])
            .split(frame.size());

        // Header
        self.render_header(frame, chunks[0]);

        // Message bar
        if let Some(ref message) = self.message {
            let msg_color = if message.contains("Error") || message.contains("already") {
                Color::Red
            } else {
                Color::Green
            };
            
            let msg_widget = Paragraph::new(message.as_str())
                .style(Style::default().fg(msg_color).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);
            frame.render_widget(msg_widget, chunks[1]);
        }

        // Main content - render based on current screen
        match &self.current_screen {
            Screen::Home => self.render_home(frame, chunks[2]),
            Screen::SessionList => self.render_session_list(frame, chunks[2]),
            Screen::SessionDetail(id) => self.render_session_detail(frame, chunks[2], *id),
            Screen::SessionCreate => self.render_session_create(frame, chunks[2]),
            Screen::SessionEdit(_) => self.render_session_edit(frame),
            Screen::SessionDelete(_) => self.render_session_delete(frame),
            Screen::TrainingContentCreate(session_id) => self.render_training_content_create(frame, chunks[2], *session_id),
            Screen::TrainingContentEdit(content_id) => self.render_training_content_edit(frame, chunks[2], *content_id),
            Screen::TrainingContentDelete(content_id) => self.render_training_content_delete(frame, *content_id),
            Screen::Help => self.render_help(frame, chunks[2]),
        }

        // Footer with dynamic help text
        self.render_footer(frame, chunks[3]);
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
        use crate::ui::help::HelpScreen;

        // Get dynamic footer help based on current screen
        let commands = HelpScreen::get_footer_help(&self.current_screen, &self.user_context);
        
        let footer_text = commands
            .iter()
            .map(|(key, desc)| format!("{} {} ", key, desc))
            .collect::<String>();

        let footer = Paragraph::new(footer_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Left);

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
            "Manage Sessions".to_string()
        } else {
            format!("{}", self.session_filter.as_str())
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
                } else if self.session_filter == SessionFilter::MySubscriptions {
                    "You haven't subscribed to any sessions yet. Press [f] to view all available sessions."
                } else {
                    "No sessions available"
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
            .map(|(i, session_with_sub)| {
                let session = &session_with_sub.session;
                
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

                // Add subscription indicators for players
                let status_indicator = if self.user_context.is_player() {
                    if session_with_sub.is_completed() {
                        " ✓"
                    } else if session_with_sub.is_subscribed() {
                        " ●"
                    } else {
                        ""
                    }
                } else {
                    ""
                };

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
                        Span::raw(format!(" - {} {}{}{}", date_str, time_str, duration_str, status_indicator)),
                    ])
                } else {
                    Line::from(format!(
                        "  {} - {} {}{}{}",
                        session.title, date_str, time_str, duration_str, status_indicator
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
        let session_with_sub = self.sessions.iter().find(|s| s.session.id == session_id);

        let content = if let Some(sws) = session_with_sub {
            let session = &sws.session;
            
            let mut lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    &session.title,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
            ];

            // Show subscription status for players
            if self.user_context.is_player() {
                if sws.is_completed() {
                    lines.push(Line::from(Span::styled(
                        "Status: ✓ Completed",
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    )));
                } else if sws.is_subscribed() {
                    lines.push(Line::from(Span::styled(
                        "Status: ● Subscribed",
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    )));
                } else {
                    lines.push(Line::from(Span::styled(
                        "Status: Not subscribed",
                        Style::default().fg(Color::DarkGray),
                    )));
                }
                lines.push(Line::from(""));
            }

            lines.extend(vec![
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
                        .unwrap_or("No description")
                ),
                Line::from(""),
            ]);

            // Display training content
            if !self.training_content.is_empty() {
                lines.push(Line::from(Span::styled(
                    "Training Content:",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                )));

                for content_item in &self.training_content {
                    lines.push(Line::from(format!(
                        "  {} - {} ({})",
                        match &content_item.content_type {
                            crate::models::ContentType::Drill => "🎯 Drill",
                            crate::models::ContentType::Exercise => "💪 Exercise",
                            crate::models::ContentType::Warmup => "🔥 Warm-up",
                            crate::models::ContentType::Cooldown => "❄️  Cool-down",
                        },
                        content_item.title,
                        content_item
                            .duration_minutes
                            .map(|d| format!("{}min", d))
                            .unwrap_or_else(|| "N/A".to_string())
                    )));

                    if let Some(desc) = &content_item.description {
                        lines.push(Line::from(format!("     {}", desc)));
                    }
                }
                lines.push(Line::from(""));
            }

            if self.user_context.is_player() {
                lines.push(Line::from(Span::styled(
                    if sws.is_completed() {
                        "This session is already marked as complete"
                    } else if sws.is_subscribed() {
                        "Press [m] to mark this session as complete"
                    } else {
                        "Subscribe to this session from the session list to track your progress"
                    },
                    Style::default().fg(Color::DarkGray),
                )));
            } else if self.training_content.is_empty() {
                lines.push(Line::from(Span::styled(
                    "No training content added yet. Use coaching tools to add drills, exercises, and quizzes.",
                    Style::default().fg(Color::DarkGray),
                )));
            }

            lines
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
        let title = "Create New Session";
        let form = &self.session_form;

        // Split area into form area and info area
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(20),
                Constraint::Length(3),
            ])
            .split(area);

        // Form fields
        let mut form_lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Title: ",
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(&form.title),
                if form.focus_field == crate::ui::session_form::FormField::Title {
                    Span::styled("▼", Style::default().fg(Color::Green))
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(vec![
                Span::styled(
                    "Description: ",
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(&form.description),
                if form.focus_field == crate::ui::session_form::FormField::Description {
                    Span::styled("▼", Style::default().fg(Color::Green))
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(vec![
                Span::styled(
                    "Date (YYYY-MM-DD): ",
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(&form.scheduled_date),
                if form.focus_field == crate::ui::session_form::FormField::ScheduledDate {
                    Span::styled("▼", Style::default().fg(Color::Green))
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(vec![
                Span::styled(
                    "Time (HH:MM): ",
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(&form.scheduled_time),
                if form.focus_field == crate::ui::session_form::FormField::ScheduledTime {
                    Span::styled("▼", Style::default().fg(Color::Green))
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(vec![
                Span::styled(
                    "Duration (minutes): ",
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(&form.duration_minutes),
                if form.focus_field == crate::ui::session_form::FormField::DurationMinutes {
                    Span::styled("▼", Style::default().fg(Color::Green))
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(vec![
                Span::styled(
                    "Skill Level: ",
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(&form.skill_level),
                if form.focus_field == crate::ui::session_form::FormField::SkillLevel {
                    Span::styled("▼", Style::default().fg(Color::Green))
                } else {
                    Span::raw("")
                },
            ]),
            Line::from(""),
        ];

        let paragraph = Paragraph::new(form_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .style(Style::default().fg(Color::White)),
            )
            .alignment(Alignment::Left);

        frame.render_widget(paragraph, chunks[0]);

        // Help text
        let help_text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("[Tab]", Style::default().fg(Color::Cyan)),
                Span::raw(" Next field  "),
                Span::styled("[Shift+Tab]", Style::default().fg(Color::Cyan)),
                Span::raw(" Prev field  "),
                Span::styled("[Enter]", Style::default().fg(Color::Cyan)),
                Span::raw(" Save  "),
                Span::styled("[Esc]", Style::default().fg(Color::Cyan)),
                Span::raw(" Cancel"),
            ]),
        ];

        let help_para = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::BOTTOM))
            .alignment(Alignment::Left);

        frame.render_widget(help_para, chunks[1]);
    }

    fn render_session_edit(&self, frame: &mut Frame) {
        let size = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
            .split(size);

        // Header
        self.render_header(frame, chunks[0]);

        // Form content
        if let Some(form) = &self.session_edit_form {
            let form_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .margin(2)
                .split(chunks[1]);

            // Title field
            let title_block = Block::default()
                .title("Title")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(if form.focus_field == crate::ui::session_edit_form::FormField::Title {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                });
            let title_para = Paragraph::new(form.title.as_str()).block(title_block);
            frame.render_widget(title_para, form_chunks[0]);

            // Description field
            let desc_block = Block::default()
                .title("Description (Optional)")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(if form.focus_field == crate::ui::session_edit_form::FormField::Description {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                });
            let desc_para = Paragraph::new(form.description.as_str()).block(desc_block);
            frame.render_widget(desc_para, form_chunks[1]);

            // Date field
            let date_block = Block::default()
                .title("Date (YYYY-MM-DD)")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(if form.focus_field == crate::ui::session_edit_form::FormField::ScheduledDate {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                });
            let date_para = Paragraph::new(form.scheduled_date.as_str()).block(date_block);
            frame.render_widget(date_para, form_chunks[2]);

            // Time field
            let time_block = Block::default()
                .title("Time (HH:MM)")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(if form.focus_field == crate::ui::session_edit_form::FormField::ScheduledTime {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                });
            let time_para = Paragraph::new(form.scheduled_time.as_str()).block(time_block);
            frame.render_widget(time_para, form_chunks[3]);

            // Duration field
            let duration_block = Block::default()
                .title("Duration (minutes, 5-480)")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(if form.focus_field == crate::ui::session_edit_form::FormField::DurationMinutes {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                });
            let duration_para = Paragraph::new(form.duration_minutes.as_str()).block(duration_block);
            frame.render_widget(duration_para, form_chunks[4]);

            // Skill level field
            let skill_block = Block::default()
                .title("Skill Level (←/→ to cycle)")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .style(if form.focus_field == crate::ui::session_edit_form::FormField::SkillLevel {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                });
            let skill_para = Paragraph::new(form.skill_level.as_str()).block(skill_block);
            frame.render_widget(skill_para, form_chunks[5]);
        }

        // Footer with help
        let help_text = vec![
            Line::from(vec![
                Span::styled("[Tab] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Next field  "),
                Span::styled("[Shift+Tab] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Previous field  "),
                Span::styled("[Enter] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Save  "),
                Span::styled("[Esc] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Cancel"),
            ]),
        ];

        let help_para = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::BOTTOM))
            .alignment(Alignment::Left);

        frame.render_widget(help_para, chunks[2]);
    }

    fn render_session_delete(&self, frame: &mut Frame) {
        let size = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
            .split(size);

        // Header
        self.render_header(frame, chunks[0]);

        // Delete confirmation dialog
        let dialog_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(40), Constraint::Percentage(30)])
            .split(chunks[1]);

        let confirm_text = vec![
            Line::from(""),
            Line::from(Span::styled("Delete this session?", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(Span::raw("All associated subscriptions will also be deleted.")),
            Line::from(""),
        ];

        let confirm_para = Paragraph::new(confirm_text)
            .block(Block::default().borders(Borders::ALL).title("Confirmation"))
            .alignment(Alignment::Center);

        frame.render_widget(confirm_para, dialog_chunks[1]);

        // Footer
        let help_text = vec![
            Line::from(vec![
                Span::styled("[y] ", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
                Span::raw("Confirm  "),
                Span::styled("[n] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Cancel"),
            ]),
        ];

        let help_para = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::BOTTOM))
            .alignment(Alignment::Left);

        frame.render_widget(help_para, chunks[2]);
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        use crate::ui::help::HelpScreen;

        let help_text = HelpScreen::get_help_text();
        
        let text_lines: Vec<Line> = help_text
            .iter()
            .map(|line| {
                if line.starts_with("═") {
                    Line::from(Span::styled(line.clone(), Style::default().fg(Color::Cyan)))
                } else if line.starts_with("GLOBAL") || line.starts_with("NAVIGATION") 
                    || line.starts_with("SESSION") || line.starts_with("DELETION") 
                    || line.starts_with("FORM") || line.starts_with("TEXT") 
                    || line.starts_with("FIELD") {
                    Line::from(Span::styled(line.clone(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
                } else {
                    Line::from(line.clone())
                }
            })
            .collect();

        let help_para = Paragraph::new(text_lines)
            .block(Block::default().title("Help - Keyboard Commands").borders(Borders::ALL))
            .alignment(Alignment::Left);

        frame.render_widget(help_para, area);
    }

    fn render_training_content_create(&self, frame: &mut Frame, area: Rect, _session_id: i64) {
        let form = &self.training_content_form;
        
        // Layout for form fields
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Min(1),
            ])
            .split(area);

        // Title field
        let title_block = Block::default()
            .title("Title (required, 2-100 chars)")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(if form.focus_field == crate::ui::training_content_form::FormField::Title {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });
        let title_para = Paragraph::new(form.title.as_str()).block(title_block);
        frame.render_widget(title_para, chunks[1]);

        // Description field
        let desc_block = Block::default()
            .title("Description (optional, max 500 chars)")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(if form.focus_field == crate::ui::training_content_form::FormField::Description {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });
        let desc_para = Paragraph::new(form.description.as_str()).block(desc_block);
        frame.render_widget(desc_para, chunks[2]);

        // Duration field
        let duration_block = Block::default()
            .title("Duration in minutes (optional, 1-480)")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(if form.focus_field == crate::ui::training_content_form::FormField::DurationMinutes {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });
        let duration_para = Paragraph::new(form.duration_minutes.as_str()).block(duration_block);
        frame.render_widget(duration_para, chunks[3]);

        // Content type field
        let content_type_block = Block::default()
            .title("Content Type (←/→ to cycle)")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(if form.focus_field == crate::ui::training_content_form::FormField::ContentType {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });
        let content_type_para = Paragraph::new(form.content_type.as_str()).block(content_type_block);
        frame.render_widget(content_type_para, chunks[4]);

        // Footer with help
        let help_text = vec![
            Line::from(vec![
                Span::styled("[Tab] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Next field  "),
                Span::styled("[Shift+Tab] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Previous field  "),
                Span::styled("[Enter] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Save  "),
                Span::styled("[Esc] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Cancel"),
            ]),
        ];
        let help_para = Paragraph::new(help_text);
        frame.render_widget(help_para, chunks[5]);
    }

    fn render_training_content_edit(&self, frame: &mut Frame, area: Rect, _content_id: i64) {
        let form = &self.training_content_form;
        
        // Layout for form fields
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Length(4),
                Constraint::Min(1),
            ])
            .split(area);

        // Title field
        let title_block = Block::default()
            .title("Title (required, 2-100 chars)")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(if form.focus_field == crate::ui::training_content_form::FormField::Title {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });
        let title_para = Paragraph::new(form.title.as_str()).block(title_block);
        frame.render_widget(title_para, chunks[1]);

        // Description field
        let desc_block = Block::default()
            .title("Description (optional, max 500 chars)")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(if form.focus_field == crate::ui::training_content_form::FormField::Description {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });
        let desc_para = Paragraph::new(form.description.as_str()).block(desc_block);
        frame.render_widget(desc_para, chunks[2]);

        // Duration field
        let duration_block = Block::default()
            .title("Duration in minutes (optional, 1-480)")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(if form.focus_field == crate::ui::training_content_form::FormField::DurationMinutes {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });
        let duration_para = Paragraph::new(form.duration_minutes.as_str()).block(duration_block);
        frame.render_widget(duration_para, chunks[3]);

        // Content type field
        let content_type_block = Block::default()
            .title("Content Type (←/→ to cycle)")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(if form.focus_field == crate::ui::training_content_form::FormField::ContentType {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });
        let content_type_para = Paragraph::new(form.content_type.as_str()).block(content_type_block);
        frame.render_widget(content_type_para, chunks[4]);

        // Footer with help
        let help_text = vec![
            Line::from(vec![
                Span::styled("[Tab] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Next field  "),
                Span::styled("[Shift+Tab] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Previous field  "),
                Span::styled("[Enter] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Update  "),
                Span::styled("[Esc] ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Cancel"),
            ]),
        ];
        let help_para = Paragraph::new(help_text);
        frame.render_widget(help_para, chunks[5]);
    }

    fn render_training_content_delete(&self, frame: &mut Frame, content_id: i64) {
        // Find the content to get details
        if let Some(content) = self.training_content.iter().find(|c| c.id == content_id) {
            let lines = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "Confirm deletion of training content?",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(format!("Title: {}", content.title)),
                Line::from(format!("Type: {:?}", content.content_type)),
                Line::from(""),
                Line::from(Span::styled(
                    "Press [y] to confirm or [n] to cancel",
                    Style::default().fg(Color::Yellow),
                )),
            ];
            let para = Paragraph::new(lines)
                .block(Block::default().title("Delete Training Content").borders(Borders::ALL))
                .alignment(Alignment::Left);
            frame.render_widget(para, frame.size());
        } else {
            let msg = Paragraph::new("Content not found")
                .block(Block::default().title("Error").borders(Borders::ALL))
                .alignment(Alignment::Center);
            frame.render_widget(msg, frame.size());
        }
    }
}