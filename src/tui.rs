use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event as CEvent, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
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
    let tick_rate = Duration::from_millis(160); // ~6.2 FPS smooth animation loop
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

            // Sync from daemon every 1 second (every 4 ticks)
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
    let size = f.area();
    let is_online = is_daemon_running();

    // Mini mode for tiny tmux/zellij splits (width < 45 or height < 12)
    if size.width < 45 || size.height < 12 {
        render_compact_ui(f, size, state, tick, is_online);
        return;
    }

    // Standard mode
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(8), // Pet & Status
            Constraint::Length(8), // Gauges & Stats
            Constraint::Length(1), // Footer / Hotkeys
        ])
        .split(size);

    render_pet_header(f, chunks[0], state, tick, is_online);
    render_gauges(f, chunks[1], state);
    render_footer(f, chunks[2]);
}

fn render_compact_ui(f: &mut Frame, area: Rect, state: &PetState, tick: usize, is_online: bool) {
    let anim = get_animation_frame(state.status, tick);
    let mut text = anim.lines;

    let status_dot = if is_online {
        Span::styled("●", Style::default().fg(Color::Rgb(166, 227, 161)))
    } else {
        Span::styled("○", Style::default().fg(Color::Rgb(243, 139, 168)))
    };

    text.push(Line::from(vec![
        status_dot,
        Span::raw(" "),
        Span::styled(format!("Lv.{} {}", state.level, state.status.emoji()), Style::default().fg(Color::Yellow)),
    ]));

    let widget = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(anim.primary_color))
                .title("🐾 Chiv"),
        );

    f.render_widget(widget, area);
}

fn render_pet_header(f: &mut Frame, area: Rect, state: &PetState, tick: usize, is_online: bool) {
    let anim = get_animation_frame(state.status, tick);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(28), // Sprite
            Constraint::Min(20),   // Info
        ])
        .split(area);

    let sprite_widget = Paragraph::new(anim.lines)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(anim.primary_color))
                .title("Pet"),
        );
    f.render_widget(sprite_widget, chunks[0]);

    // Info Block
    let status_dot = if is_online {
        Span::styled("● online", Style::default().fg(Color::Rgb(166, 227, 161)))
    } else {
        Span::styled("○ offline", Style::default().fg(Color::Rgb(243, 139, 168)))
    };

    let next_xp = state.xp_for_next_level();
    let info_lines = vec![
        Line::from(vec![
            Span::styled("🐾 ", Style::default()),
            Span::styled(state.name.clone(), Style::default().add_modifier(Modifier::BOLD)),
            Span::styled(format!(" (Lv.{})  [", state.level), Style::default().fg(Color::Yellow)),
            status_dot,
            Span::styled("]", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::DarkGray)),
            Span::raw(format!("{} ", state.status.emoji())),
            Span::styled(
                format!("{:?}", state.status),
                Style::default().fg(anim.primary_color).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(&state.status_message, Style::default().fg(Color::Rgb(205, 214, 244))),
        ]),
        Line::from(vec![
            Span::styled("XP: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}/{}", state.xp, next_xp), Style::default().fg(Color::Rgb(137, 180, 250))),
        ]),
    ];

    let info_widget = Paragraph::new(info_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Rgb(69, 71, 90)))
                .title("Companion Status"),
        );
    f.render_widget(info_widget, chunks[1]);
}

fn render_gauges(f: &mut Frame, area: Rect, state: &PetState) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(69, 71, 90)))
        .title("Vitals & Progress");

    let inner = block.inner(area);
    f.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Satiety
            Constraint::Length(1), // Mood
            Constraint::Length(1), // Energy
            Constraint::Length(1), // XP
            Constraint::Length(1), // Stats
        ])
        .split(inner);

    // Satiety Gauge
    let satiety_gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Rgb(250, 179, 135)))
        .ratio(state.satiety as f64 / 100.0)
        .label(format!("Satiety: {}/100", state.satiety));
    f.render_widget(satiety_gauge, rows[0]);

    // Mood Gauge
    let mood_gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Rgb(166, 227, 161)))
        .ratio(state.mood as f64 / 100.0)
        .label(format!("Mood: {}/100", state.mood));
    f.render_widget(mood_gauge, rows[1]);

    // Energy Gauge
    let energy_gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Rgb(249, 226, 175)))
        .ratio(state.energy as f64 / 100.0)
        .label(format!("Energy: {}/100", state.energy));
    f.render_widget(energy_gauge, rows[2]);

    // XP Gauge
    let next_xp = state.xp_for_next_level();
    let xp_ratio = if next_xp > 0 {
        (state.xp as f64 / next_xp as f64).min(1.0)
    } else {
        0.0
    };
    let xp_gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Rgb(137, 180, 250)))
        .ratio(xp_ratio)
        .label(format!("XP Progress: {}/{}", state.xp, next_xp));
    f.render_widget(xp_gauge, rows[3]);

    // Command Stats
    let stats_text = Line::from(vec![
        Span::styled("Commands: ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{}", state.total_commands), Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" ("),
        Span::styled(format!("{} ok", state.successful_commands), Style::default().fg(Color::Green)),
        Span::raw(", "),
        Span::styled(format!("{} failed", state.failed_commands), Style::default().fg(Color::Red)),
        Span::raw(")"),
    ]);
    f.render_widget(Paragraph::new(stats_text).alignment(Alignment::Center), rows[4]);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let footer_text = Line::from(vec![
        Span::styled("[F]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("eed 🐟   "),
        Span::styled("[P]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("et 🐾   "),
        Span::styled("[S]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("leep/Wake 💤   "),
        Span::styled("[R]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("efresh   "),
        Span::styled("[Q]", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::raw("uit"),
    ]);

    let footer = Paragraph::new(footer_text).alignment(Alignment::Center);
    f.render_widget(footer, area);
}
