use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PetStatus {
    Sleeping,
    Idle,
    Focused,
    Cheering,
    Frustrated,
    Eating,
    Hacking,
}

impl PetStatus {
    pub fn emoji(&self) -> &'static str {
        match self {
            PetStatus::Sleeping => "💤",
            PetStatus::Idle => "🐱",
            PetStatus::Focused => "🧐",
            PetStatus::Cheering => "🎉",
            PetStatus::Frustrated => "😿",
            PetStatus::Eating => "🐟",
            PetStatus::Hacking => "💻",
        }
    }

    pub fn ascii_art(&self) -> &'static str {
        match self {
            PetStatus::Sleeping => "    /\\_/\\\n   ( -.- )  zZz\n  / (   ) \\\n (____)____)\n            ",
            PetStatus::Idle => "    /\\_/\\\n   ( o.o )\n   /     \\   ~\n  (  ( )  )\n   (__)_(__)",
            PetStatus::Focused => "    /\\_/\\\n   ( O.O )  !\n  / (   ) \\\n (  ( )  )\n  (__)_(__)",
            PetStatus::Cheering => "    /\\_/\\   ✨\n  \\( ^.^ )/  🎉\n   /(   )\\   *purr*\n  (__)_(__)\n            ",
            PetStatus::Frustrated => "    \\___/\n   ( T.T )  💧\n  / (   ) \\\n (  ( )  )\n  (__)_(__)",
            PetStatus::Eating => "    /\\_/\\\n   ( >.< )~🐟\n  / (o)o \\  *nom*\n (  ( )  )\n  (__)_(__)",
            PetStatus::Hacking => "    /\\_/\\\n   ( •̀ ω •́ )   💻\n  /  (   \\__ [===] tap!\n (____)____(__)\n            ",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetState {
    pub name: String,
    pub level: u32,
    pub xp: u32,
    pub satiety: u32,
    pub mood: u32,
    pub energy: u32,
    pub status: PetStatus,
    pub status_message: String,
    pub status_until: Option<u64>,
    pub last_interaction_ts: u64,
    pub last_decay_ts: u64,
    pub total_commands: u64,
    pub successful_commands: u64,
    pub failed_commands: u64,
}

impl Default for PetState {
    fn default() -> Self {
        let now = now_secs();
        Self {
            name: "Chiv".to_string(),
            level: 1,
            xp: 0,
            satiety: 80,
            mood: 85,
            energy: 90,
            status: PetStatus::Idle,
            status_message: "Woke up and ready to code!".to_string(),
            status_until: None,
            last_interaction_ts: now,
            last_decay_ts: now,
            total_commands: 0,
            successful_commands: 0,
            failed_commands: 0,
        }
    }
}

impl PetState {
    pub fn xp_for_next_level(&self) -> u32 {
        self.level * 100
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    ShellPreExec {
        cmd: String,
    },
    ShellPostExec {
        cmd: String,
        exit_code: i32,
        #[serde(default)]
        duration_ms: Option<u64>,
    },
    AgentAction {
        agent: String,
        action: String,
        #[serde(default)]
        detail: Option<String>,
    },
    Feed {
        #[serde(default)]
        item: Option<String>,
    },
    Pet,
    Wake,
    Sleep,
}

pub fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
