use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};

use crate::daemon::run_daemon;
use crate::ipc::{is_daemon_running, send_request, IpcRequest};
use crate::model::{Event, PetState};
use crate::storage::load_state;

#[derive(Parser, Debug)]
#[command(name = "chivpet")]
#[command(about = "Terminal Pet companion for developer workflows", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start the background daemon
    Daemon {
        /// Run in foreground
        #[arg(short, long, default_value_t = true)]
        foreground: bool,
    },
    /// Display current pet status
    Status {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Feed the pet some food
    Feed {
        /// Optional food name (e.g. fish, sushi, cookie)
        item: Option<String>,
    },
    /// Pet your companion to cheer it up
    Pet,
    /// Wake up the pet from sleep
    Wake,
    /// Put the pet to sleep
    Sleep,
    /// Send an event to the pet (used by shell hooks and agent integrations)
    Event {
        #[command(subcommand)]
        action: EventSubcommand,
    },
    /// Generate shell hook integration script
    Hook {
        /// Target shell (zsh, bash, fish)
        shell: ShellType,
    },
    /// Open interactive animated TUI view (for tmux / splits)
    Tui,
}

#[derive(Subcommand, Debug)]
pub enum EventSubcommand {
    /// Pre-execution event before running a command
    PreExec {
        /// The command being executed
        cmd: String,
    },
    /// Post-execution event after running a command
    PostExec {
        /// The command that finished
        cmd: String,
        /// Exit code of the command
        #[arg(short, long, default_value_t = 0)]
        exit_code: i32,
        /// Execution duration in milliseconds
        #[arg(short, long)]
        duration_ms: Option<u64>,
    },
    /// Event triggered by AI coding agents (Codex, Claude Code, etc.)
    Agent {
        /// Agent name (e.g. codex, claude, opencode)
        agent: String,
        /// Action performed (e.g. tool_call, thinking, done)
        action: String,
        /// Optional details or parameters
        #[arg(short, long)]
        detail: Option<String>,
    },
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellType {
    Zsh,
    Bash,
    Fish,
}

pub async fn run_cli(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Daemon { foreground } => {
            run_daemon(foreground).await?;
        }
        Commands::Status { json } => {
            handle_status(json)?;
        }
        Commands::Feed { item } => {
            handle_feed(item)?;
        }
        Commands::Pet => {
            handle_pet()?;
        }
        Commands::Wake => {
            handle_wake()?;
        }
        Commands::Sleep => {
            handle_sleep()?;
        }
        Commands::Event { action } => {
            handle_event(action)?;
        }
        Commands::Hook { shell } => {
            handle_hook(shell);
        }
        Commands::Tui => {
            crate::tui::run_tui()?;
        }
    }
    Ok(())
}

fn handle_status(json: bool) -> Result<()> {
    let running = is_daemon_running();
    let state = if running {
        match send_request(&IpcRequest::Status) {
            Ok(resp) => resp.state.unwrap_or_else(load_state),
            Err(_) => load_state(),
        }
    } else {
        load_state()
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&state)?);
        return Ok(());
    }

    render_pet_card(&state, running);
    Ok(())
}

fn handle_feed(item: Option<String>) -> Result<()> {
    let resp = send_request(&IpcRequest::Feed { item })
        .context("Failed to communicate with chivpet daemon. Is `chivpet daemon` running?")?;
    if let Some(msg) = resp.message {
        println!("{}", msg);
    }
    Ok(())
}

fn handle_pet() -> Result<()> {
    let resp = send_request(&IpcRequest::Pet)
        .context("Failed to communicate with chivpet daemon. Is `chivpet daemon` running?")?;
    if let Some(msg) = resp.message {
        println!("{}", msg);
    }
    Ok(())
}

fn handle_wake() -> Result<()> {
    let _ = send_request(&IpcRequest::Wake)
        .context("Failed to communicate with chivpet daemon. Is `chivpet daemon` running?")?;
    println!("🐾 Pet has been woken up!");
    Ok(())
}

fn handle_sleep() -> Result<()> {
    let _ = send_request(&IpcRequest::Sleep)
        .context("Failed to communicate with chivpet daemon. Is `chivpet daemon` running?")?;
    println!("🐾 Pet is now sleeping. Sweet dreams!");
    Ok(())
}

fn handle_event(action: EventSubcommand) -> Result<()> {
    let event = match action {
        EventSubcommand::PreExec { cmd } => Event::ShellPreExec { cmd },
        EventSubcommand::PostExec {
            cmd,
            exit_code,
            duration_ms,
        } => Event::ShellPostExec {
            cmd,
            exit_code,
            duration_ms,
        },
        EventSubcommand::Agent {
            agent,
            action,
            detail,
        } => Event::AgentAction {
            agent,
            action,
            detail,
        },
    };

    // If daemon is not running, we fail silently or exit cleanly to avoid breaking shell hooks
    if !is_daemon_running() {
        return Ok(());
    }

    let _ = send_request(&IpcRequest::Event { event });
    Ok(())
}

fn render_pet_card(state: &PetState, daemon_online: bool) {
    let ascii_lines: Vec<&str> = state.status.ascii_art().lines().collect();
    let l0 = ascii_lines.first().unwrap_or(&"");
    let l1 = ascii_lines.get(1).unwrap_or(&"");
    let l2 = ascii_lines.get(2).unwrap_or(&"");
    let l3 = ascii_lines.get(3).unwrap_or(&"");
    let l4 = ascii_lines.get(4).unwrap_or(&"");

    let daemon_tag = if daemon_online {
        "\x1b[32m● online\x1b[0m"
    } else {
        "\x1b[31m○ offline (using saved state)\x1b[0m"
    };

    let next_xp = state.xp_for_next_level();
    let xp_bar = make_bar(state.xp, next_xp, 12);
    let sat_bar = make_bar(state.satiety, 100, 8);
    let mood_bar = make_bar(state.mood, 100, 8);
    let nrg_bar = make_bar(state.energy, 100, 8);

    println!();
    println!("  {:<24}  🐾 \x1b[1m{}\x1b[0m (Lv.{}) [{}]", l0, state.name, state.level, daemon_tag);
    println!("  {:<24}  Status: {} \x1b[1m{:?}\x1b[0m - {}", l1, state.status.emoji(), state.status, state.status_message);
    println!("  {:<24}  XP:     [{}] {}/{}", l2, xp_bar, state.xp, next_xp);
    println!("  {:<24}  Satiety:[{}] {:>3}  Mood:[{}] {:>3}  Energy:[{}] {:>3}", l3, sat_bar, state.satiety, mood_bar, state.mood, nrg_bar, state.energy);
    println!("  {:<24}  Commands: {} total ({} ok, {} failed)", l4, state.total_commands, state.successful_commands, state.failed_commands);
    println!();
}

fn make_bar(value: u32, max: u32, width: usize) -> String {
    if max == 0 {
        return " ".repeat(width);
    }
    let filled = ((value as f64 / max as f64) * width as f64).round() as usize;
    let filled = filled.min(width);
    let empty = width.saturating_sub(filled);

    format!("{}{}", "=".repeat(filled), " ".repeat(empty))
}

fn handle_hook(shell: ShellType) {
    match shell {
        ShellType::Zsh => {
            print!("{}", r#"# chivpet zsh integration
# Add `eval "$(chivpet hook zsh)"` to ~/.zshrc

_chivpet_preexec() {
    chivpet event pre-exec "$1" >/dev/null 2>&1 &!
}

_chivpet_precmd() {
    local exit_code=$?
    chivpet event post-exec "$history[$((HISTCMD-1))]" --exit-code $exit_code >/dev/null 2>&1 &!
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec _chivpet_preexec
add-zsh-hook precmd _chivpet_precmd
"#);
        }
        ShellType::Bash => {
            print!("{}", r#"# chivpet bash integration
# Add `eval "$(chivpet hook bash)"` to ~/.bashrc

_chivpet_precmd() {
    local exit_code=$?
    local last_cmd=$(HISTTIMEFORMAT= history 1 | sed 's/^[ ]*[0-9]*[ ]*//')
    (chivpet event post-exec "$last_cmd" --exit-code $exit_code >/dev/null 2>&1 &)
}

if [[ ! "$PROMPT_COMMAND" =~ "_chivpet_precmd" ]]; then
    PROMPT_COMMAND="_chivpet_precmd${PROMPT_COMMAND:+;$PROMPT_COMMAND}"
fi
"#);
        }
        ShellType::Fish => {
            print!("{}", r#"# chivpet fish integration
# Add `chivpet hook fish | source` to ~/.config/fish/config.fish

function __chivpet_postexec --on-event fish_postexec
    set -l exit_code $status
    chivpet event post-exec "$argv" --exit-code $exit_code >/dev/null 2>&1 &
end
"#);
        }
    }
}
