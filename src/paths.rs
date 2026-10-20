use std::path::PathBuf;

#[allow(dead_code)]
pub fn config_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(dir).join("chivpet")
    } else if let Some(home) = dirs::home_dir() {
        home.join(".config").join("chivpet")
    } else {
        PathBuf::from(".chivpet_config")
    }
}

pub fn data_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("XDG_DATA_HOME") {
        PathBuf::from(dir).join("chivpet")
    } else if let Some(home) = dirs::home_dir() {
        home.join(".local").join("share").join("chivpet")
    } else {
        PathBuf::from(".chivpet_data")
    }
}

pub fn state_file_path() -> PathBuf {
    data_dir().join("state.json")
}

pub fn socket_path() -> PathBuf {
    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        PathBuf::from(runtime_dir).join("chivpet.sock")
    } else {
        let uid = libc_getuid_fallback();
        PathBuf::from(format!("/tmp/chivpet-{}.sock", uid))
    }
}

fn libc_getuid_fallback() -> u32 {
    std::env::var("UID")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000)
}
