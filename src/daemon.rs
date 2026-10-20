use anyhow::{Context, Result};
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::Mutex;
use tokio::time::interval;

use crate::engine::PetEngine;
use crate::ipc::{is_daemon_running, IpcRequest, IpcResponse};
use crate::model::Event;
use crate::paths::socket_path;
use crate::storage::{load_state, save_state};

pub async fn run_daemon(foreground: bool) -> Result<()> {
    let sock = socket_path();

    if is_daemon_running() {
        eprintln!("chivpet daemon is already running on {}", sock.display());
        return Ok(());
    }

    if sock.exists() {
        let _ = fs::remove_file(&sock);
    }

    if let Some(parent) = sock.parent() {
        fs::create_dir_all(parent)?;
    }

    let initial_state = load_state();
    let engine = Arc::new(Mutex::new(PetEngine::new(initial_state)));

    let listener = UnixListener::bind(&sock)
        .with_context(|| format!("Failed to bind Unix socket at {}", sock.display()))?;

    if foreground {
        println!("🐾 chivpet daemon started in foreground.");
        println!("   Listening on: {}", sock.display());
        println!("   Press Ctrl+C to stop.");
    }

    // 1. Periodic tick task (every 1 second)
    let engine_tick = Arc::clone(&engine);
    let tick_handle = tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(1));
        let mut save_counter = 0u32;
        loop {
            ticker.tick().await;
            {
                let mut eng = engine_tick.lock().await;
                eng.tick();
                save_counter += 1;
                if save_counter >= 10 {
                    save_counter = 0;
                    if let Err(e) = save_state(eng.state()) {
                        eprintln!("[warn] Failed to autosave state: {}", e);
                    }
                }
            }
        }
    });

    // 2. Main accept loop & signal handler
    let engine_accept = Arc::clone(&engine);
    let sock_clone = sock.clone();

    tokio::select! {
        _ = accept_loop(listener, engine_accept) => {},
        _ = tokio::signal::ctrl_c() => {
            if foreground {
                println!("\n🐾 Received shutdown signal, stopping daemon...");
            }
        }
    }

    // Abort tick task
    tick_handle.abort();

    // Final state save
    {
        let eng = engine.lock().await;
        if let Err(e) = save_state(eng.state()) {
            eprintln!("[warn] Failed to save state on exit: {}", e);
        }
    }

    // Clean up socket file
    if sock_clone.exists() {
        let _ = fs::remove_file(&sock_clone);
    }

    if foreground {
        println!("🐾 chivpet daemon exited cleanly.");
    }

    Ok(())
}

async fn accept_loop(listener: UnixListener, engine: Arc<Mutex<PetEngine>>) -> Result<()> {
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let engine_conn = Arc::clone(&engine);
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream, engine_conn).await {
                        eprintln!("[warn] Client connection error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("[warn] Failed to accept connection: {}", e);
            }
        }
    }
}

async fn handle_client(mut stream: UnixStream, engine: Arc<Mutex<PetEngine>>) -> Result<()> {
    let (reader, mut writer) = stream.split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    if buf_reader.read_line(&mut line).await? == 0 {
        return Ok(());
    }

    let req: IpcRequest = match serde_json::from_str(line.trim()) {
        Ok(r) => r,
        Err(e) => {
            let resp = IpcResponse::err(format!("Invalid request JSON: {}", e));
            let mut out = serde_json::to_string(&resp)?;
            out.push('\n');
            writer.write_all(out.as_bytes()).await?;
            return Ok(());
        }
    };

    let response = {
        let mut eng = engine.lock().await;
        match req {
            IpcRequest::Ping => IpcResponse::ok_with_msg("pong", None),
            IpcRequest::Status => IpcResponse::ok_with_state(eng.state().clone()),
            IpcRequest::Feed { item } => {
                eng.handle_event(Event::Feed { item: item.clone() });
                let state = eng.state().clone();
                let msg = state.status_message.clone();
                let _ = save_state(&state);
                IpcResponse::ok_with_msg(msg, Some(state))
            }
            IpcRequest::Pet => {
                eng.handle_event(Event::Pet);
                let state = eng.state().clone();
                let msg = state.status_message.clone();
                let _ = save_state(&state);
                IpcResponse::ok_with_msg(msg, Some(state))
            }
            IpcRequest::Wake => {
                eng.handle_event(Event::Wake);
                let state = eng.state().clone();
                let _ = save_state(&state);
                IpcResponse::ok_with_state(state)
            }
            IpcRequest::Sleep => {
                eng.handle_event(Event::Sleep);
                let state = eng.state().clone();
                let _ = save_state(&state);
                IpcResponse::ok_with_state(state)
            }
            IpcRequest::Event { event } => {
                eng.handle_event(event);
                let state = eng.state().clone();
                let _ = save_state(&state);
                IpcResponse::ok_with_state(state)
            }
        }
    };

    let mut out = serde_json::to_string(&response)?;
    out.push('\n');
    writer.write_all(out.as_bytes()).await?;
    writer.flush().await?;

    Ok(())
}
