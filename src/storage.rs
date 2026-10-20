use anyhow::{Context, Result};
use std::fs;
use std::io::Write;

use crate::model::PetState;
use crate::paths::{data_dir, state_file_path};

pub fn load_state() -> PetState {
    let path = state_file_path();
    if !path.exists() {
        return PetState::default();
    }

    match fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str::<PetState>(&content) {
            Ok(state) => state,
            Err(e) => {
                eprintln!("[warn] Failed to parse {}: {}. Using default.", path.display(), e);
                PetState::default()
            }
        },
        Err(e) => {
            eprintln!("[warn] Failed to read {}: {}. Using default.", path.display(), e);
            PetState::default()
        }
    }
}

pub fn save_state(state: &PetState) -> Result<()> {
    let dir = data_dir();
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create data directory {}", dir.display()))?;

    let path = state_file_path();
    let tmp_path = dir.join(format!(".state.json.tmp.{}", std::process::id()));

    let json = serde_json::to_string_pretty(state)?;
    {
        let mut file = fs::File::create(&tmp_path)
            .with_context(|| format!("Failed to create temp file {}", tmp_path.display()))?;
        file.write_all(json.as_bytes())?;
        file.sync_all()?;
    }

    fs::rename(&tmp_path, &path)
        .with_context(|| format!("Failed to rename {} to {}", tmp_path.display(), path.display()))?;

    Ok(())
}
