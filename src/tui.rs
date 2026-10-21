use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event as CEvent, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame, Terminal,
};
use std::io::stdout;
use std::time::{Duration, Instant};

use crate::animation::get_animation_frame;
use crate::ipc::{is_daemon_running, send_request, IpcRequest};
use crate::model::PetState;
use crate::storage::load_state;

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, Show);
    }
}

pub fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    let _guard = TerminalGuard;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = fetch_current_state();
    let mut tick_count: usize = 0;
    let tick_rate = Duration::from_millis(90); // ~11 FPS smooth animation loop
    let mut last_tick = Instant::now();
    let mut last_fetch = Instant::now();

    loop {
        terminal.draw(|f| render_ui(f, &state, tick_count))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let CEvent::Key(key) = event::read()? {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    break;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('f') => {
                        let _ = send_request(&IpcRequest::Feed { item: Some("tuna".into()) });
                        state = fetch_current_state();
                    }
                    KeyCode::Char('p') => {
                        let _ = send_request(&IpcRequest::Pet);
                        state = fetch_current_state();
                    }
                    KeyCode::Char('s') => {
                        if state.status == crate::model::PetStatus::Sleeping {
                            let _ = send_request(&IpcRequest::Wake);
                        } else {
                            let _ = send_request(&IpcRequest::Sleep);
                        }
                        state = fetch_current_state();
                    }
                    KeyCode::Char('r') => {
                        state = fetch_current_state();
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            tick_count = tick_count.wrapping_add(1);
            last_tick = Instant::now();

            // Sync from daemon every 1 second
            if last_fetch.elapsed() >= Duration::from_secs(1) {
                state = fetch_current_state();
                last_fetch = Instant::now();
            }
        }
    }

    Ok(())
}

fn fetch_current_state() -> PetState {
    if is_daemon_running() {
        if let Ok(resp) = send_request(&IpcRequest::Status) {
            if let Some(s) = resp.state {
                return s;
            }
        }
    }
    load_state()
}

fn render_ui(f: &mut Frame, state: &PetState, tick: usize) {
    let area = f.area();
    let anim = get_animation_frame(state.status, tick);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Top flexible padding
            Constraint::Length(5), // Pet animation (centered)
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Status line ("Chiv · <message>")
            Constraint::Length(1), // Keybinding hints
            Constraint::Min(0),    // Bottom flexible padding
        ])
        .split(area);

    // 1. Pet animation (centered)
    let pet_widget = Paragraph::new(anim.lines).alignment(Alignment::Center);
    f.render_widget(pet_widget, chunks[1]);

    // 2. Status line (minimal & elegant)
    let status_line = Line::from(vec![
        Span::styled(format!("{} ", state.name), Style::default().add_modifier(Modifier::BOLD)),
        Span::styled("· ", Style::default().fg(Color::Rgb(108, 112, 134))),
        Span::styled(
            &state.status_message,
            Style::default().fg(Color::Rgb(186, 194, 222)),
        ),
    ]);
    f.render_widget(Paragraph::new(status_line).alignment(Alignment::Center), chunks[3]);

    // 3. Understated key hints
    let footer_line = Line::from(vec![
        Span::styled("f ", Style::default().fg(Color::Rgb(166, 173, 200))),
        Span::styled("feed   ", Style::default().fg(Color::Rgb(108, 112, 134))),
        Span::styled("p ", Style::default().fg(Color::Rgb(166, 173, 200))),
        Span::styled("pet   ", Style::default().fg(Color::Rgb(108, 112, 134))),
        Span::styled("s ", Style::default().fg(Color::Rgb(166, 173, 200))),
        Span::styled("sleep   ", Style::default().fg(Color::Rgb(108, 112, 134))),
        Span::styled("q ", Style::default().fg(Color::Rgb(166, 173, 200))),
        Span::styled("quit", Style::default().fg(Color::Rgb(108, 112, 134))),
    ]);
    f.render_widget(Paragraph::new(footer_line).alignment(Alignment::Center), chunks[4]);
}
