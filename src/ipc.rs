use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::time::Duration;

use crate::model::{Event, PetState};
use crate::paths::socket_path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum IpcRequest {
    Status,
    Feed {
        #[serde(default)]
        item: Option<String>,
    },
    Pet,
    Event {
        event: Event,
    },
    Wake,
    Sleep,
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<PetState>,
}

impl IpcResponse {
    pub fn ok_with_state(state: PetState) -> Self {
        Self {
            ok: true,
            message: None,
            state: Some(state),
        }
    }

    pub fn ok_with_msg(msg: impl Into<String>, state: Option<PetState>) -> Self {
        Self {
            ok: true,
            message: Some(msg.into()),
            state,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            message: Some(msg.into()),
            state: None,
        }
    }
}

pub fn send_request(req: &IpcRequest) -> Result<IpcResponse> {
    let sock = socket_path();
    let stream = UnixStream::connect(&sock)
        .with_context(|| format!("Could not connect to chivpet daemon at {}", sock.display()))?;

    stream.set_read_timeout(Some(Duration::from_millis(500)))?;
    stream.set_write_timeout(Some(Duration::from_millis(500)))?;

    let mut writer = &stream;
    let mut json = serde_json::to_string(req)?;
    json.push('\n');
    writer.write_all(json.as_bytes())?;
    writer.flush()?;

    let mut reader = BufReader::new(&stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line)?;

    if response_line.trim().is_empty() {
        anyhow::bail!("Empty response received from chivpet daemon");
    }

    let resp: IpcResponse = serde_json::from_str(&response_line)?;
    Ok(resp)
}

pub fn is_daemon_running() -> bool {
    matches!(send_request(&IpcRequest::Ping), Ok(resp) if resp.ok)
}
