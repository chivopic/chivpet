use crate::model::{now_secs, Event, PetState, PetStatus};

pub struct PetEngine {
    state: PetState,
}

impl PetEngine {
    pub fn new(state: PetState) -> Self {
        Self { state }
    }

    pub fn state(&self) -> &PetState {
        &self.state
    }

    #[allow(dead_code)]
    pub fn state_mut(&mut self) -> &mut PetState {
        &mut self.state
    }

    pub fn tick(&mut self) {
        let now = now_secs();

        // 1. Check temporary status expiration
        if let Some(until) = self.state.status_until {
            if now >= until {
                self.state.status_until = None;
                if self.is_inactive_too_long(now) {
                    self.state.status = PetStatus::Sleeping;
                    self.state.status_message = "Zzz... taking a cozy nap.".to_string();
                } else {
                    self.state.status = PetStatus::Idle;
                    self.state.status_message = "Chilling and waiting for commands.".to_string();
                }
            }
        } else if self.state.status == PetStatus::Idle && self.is_inactive_too_long(now) {
            self.state.status = PetStatus::Sleeping;
            self.state.status_message = "Zzz... taking a cozy nap.".to_string();
        }

        // 2. Periodic stat decay/recovery
        let elapsed = now.saturating_sub(self.state.last_decay_ts);
        if elapsed >= 60 {
            let minutes = elapsed / 60;
            self.state.last_decay_ts = now;

            // Satiety: -1 every 15 minutes
            let satiety_drop = (minutes / 15) as u32;
            if satiety_drop > 0 {
                self.state.satiety = self.state.satiety.saturating_sub(satiety_drop);
            }

            // Energy: if sleeping, recovers +1 every 2 minutes. If idle, -1 every 30 minutes
            if self.state.status == PetStatus::Sleeping {
                let energy_gain = (minutes / 2) as u32;
                self.state.energy = (self.state.energy + energy_gain).min(100);
            }

            // Mood penalties if starving
            if self.state.satiety == 0 {
                let mood_drop = (minutes / 5) as u32;
                self.state.mood = self.state.mood.saturating_sub(mood_drop);
            }
        }

        // 3. Level-up check
        self.check_level_up();
    }

    pub fn handle_event(&mut self, event: Event) {
        let now = now_secs();
        self.state.last_interaction_ts = now;

        // Any action wakes up a sleeping pet
        if self.state.status == PetStatus::Sleeping && !matches!(event, Event::Sleep) {
            self.state.status = PetStatus::Idle;
            self.state.status_message = "Woke up from nap!".to_string();
        }

        match event {
            Event::ShellPreExec { cmd } => {
                let trimmed = cmd.trim();
                if is_major_command(trimmed) {
                    self.state.status = PetStatus::Focused;
                    self.state.status_message = format!("Focusing on `{}`...", truncate_cmd(trimmed, 24));
                    self.state.status_until = Some(now + 8);
                }
            }
            Event::ShellPostExec { cmd, exit_code, .. } => {
                let trimmed = cmd.trim();
                self.state.total_commands += 1;

                if exit_code == 0 {
                    self.state.successful_commands += 1;
                    self.state.mood = (self.state.mood + 2).min(100);
                    self.state.xp += 5;
                    self.state.status = PetStatus::Cheering;

                    if trimmed.starts_with("git push") {
                        self.state.status_message = "🎉 Git push successful! Excellent work!".to_string();
                        self.state.xp += 10;
                    } else if trimmed.starts_with("git commit") {
                        self.state.status_message = "📦 Commit landed! Good progress!".to_string();
                        self.state.xp += 5;
                    } else if trimmed.starts_with("cargo test") || trimmed.starts_with("pytest") || trimmed.starts_with("npm test") {
                        self.state.status_message = "✅ Tests passed green!".to_string();
                        self.state.xp += 10;
                    } else {
                        self.state.status_message = format!("Nice! `{}` succeeded.", truncate_cmd(trimmed, 24));
                    }
                    self.state.status_until = Some(now + 4);
                } else {
                    self.state.failed_commands += 1;
                    self.state.mood = self.state.mood.saturating_sub(4);
                    self.state.status = PetStatus::Frustrated;
                    self.state.status_message = format!("Oops! `{}` failed (code {}).", truncate_cmd(trimmed, 24), exit_code);
                    self.state.status_until = Some(now + 5);
                }

                self.state.energy = self.state.energy.saturating_sub(1);
            }
            Event::AgentAction { agent, action, detail } => {
                self.state.status = PetStatus::Hacking;
                self.state.xp += 8;
                self.state.status_until = Some(now + 6);
                if let Some(detail) = detail {
                    self.state.status_message = format!("{}: {} ({})", agent, action, truncate_cmd(&detail, 20));
                } else {
                    self.state.status_message = format!("{}: {}", agent, action);
                }
            }
            Event::Feed { item } => {
                self.state.satiety = (self.state.satiety + 25).min(100);
                self.state.mood = (self.state.mood + 10).min(100);
                self.state.xp += 10;
                self.state.status = PetStatus::Eating;
                self.state.status_until = Some(now + 4);
                if let Some(food) = item {
                    self.state.status_message = format!("Nom nom nom! Loved the {}!", food);
                } else {
                    self.state.status_message = "Nom nom nom! Fresh tasty fish!".to_string();
                }
            }
            Event::Pet => {
                self.state.mood = (self.state.mood + 15).min(100);
                self.state.xp += 5;
                self.state.status = PetStatus::Cheering;
                self.state.status_message = "*purrs softly* Purrrrr... <3".to_string();
                self.state.status_until = Some(now + 3);
            }
            Event::Wake => {
                self.state.status = PetStatus::Idle;
                self.state.status_message = "Stretching paws... Ready!".to_string();
                self.state.status_until = None;
            }
            Event::Sleep => {
                self.state.status = PetStatus::Sleeping;
                self.state.status_message = "Going to sleep... Good night!".to_string();
                self.state.status_until = None;
            }
        }

        self.check_level_up();
    }

    fn check_level_up(&mut self) {
        while self.state.xp >= self.state.xp_for_next_level() {
            self.state.xp -= self.state.xp_for_next_level();
            self.state.level += 1;
            self.state.mood = (self.state.mood + 20).min(100);
            self.state.energy = (self.state.energy + 20).min(100);
            self.state.status = PetStatus::Cheering;
            self.state.status_message = format!("🌟 Level Up! {} is now Lv.{}!", self.state.name, self.state.level);
            self.state.status_until = Some(now_secs() + 6);
        }
    }

    fn is_inactive_too_long(&self, now: u64) -> bool {
        now.saturating_sub(self.state.last_interaction_ts) > 900 // 15 mins
    }
}

fn is_major_command(cmd: &str) -> bool {
    let lower = cmd.to_lowercase();
    lower.starts_with("git ")
        || lower.starts_with("cargo ")
        || lower.starts_with("npm ")
        || lower.starts_with("pnpm ")
        || lower.starts_with("yarn ")
        || lower.starts_with("make")
        || lower.starts_with("cmake")
        || lower.starts_with("docker ")
        || lower.starts_with("pytest")
        || lower.starts_with("python ")
        || lower.starts_with("go ")
}

fn truncate_cmd(cmd: &str, max_len: usize) -> String {
    if cmd.chars().count() <= max_len {
        cmd.to_string()
    } else {
        let mut s: String = cmd.chars().take(max_len.saturating_sub(3)).collect();
        s.push_str("...");
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feed_increases_satiety_and_mood() {
        let mut engine = PetEngine::new(PetState::default());
        engine.state.satiety = 50;
        engine.state.mood = 50;

        engine.handle_event(Event::Feed { item: Some("tuna".into()) });

        assert_eq!(engine.state.satiety, 75);
        assert_eq!(engine.state.mood, 60);
        assert_eq!(engine.state.status, PetStatus::Eating);
    }

    #[test]
    fn test_shell_success_cheers_and_adds_xp() {
        let mut engine = PetEngine::new(PetState::default());
        let initial_xp = engine.state.xp;
        let initial_mood = engine.state.mood;

        engine.handle_event(Event::ShellPostExec {
            cmd: "git push origin main".into(),
            exit_code: 0,
            duration_ms: Some(1500),
        });

        assert_eq!(engine.state.status, PetStatus::Cheering);
        assert!(engine.state.xp > initial_xp);
        assert!(engine.state.mood >= initial_mood);
        assert_eq!(engine.state.successful_commands, 1);
    }

    #[test]
    fn test_shell_failure_causes_frustration() {
        let mut engine = PetEngine::new(PetState::default());
        let initial_mood = engine.state.mood;

        engine.handle_event(Event::ShellPostExec {
            cmd: "cargo build".into(),
            exit_code: 101,
            duration_ms: Some(800),
        });

        assert_eq!(engine.state.status, PetStatus::Frustrated);
        assert!(engine.state.mood < initial_mood);
        assert_eq!(engine.state.failed_commands, 1);
    }

    #[test]
    fn test_petting_cheers_up() {
        let mut engine = PetEngine::new(PetState::default());
        engine.state.mood = 60;
        engine.handle_event(Event::Pet);
        assert_eq!(engine.state.mood, 75);
        assert_eq!(engine.state.status, PetStatus::Cheering);
    }

    #[test]
    fn test_level_up() {
        let mut engine = PetEngine::new(PetState::default());
        engine.state.level = 1;
        engine.state.xp = 95;

        // Gain 10 xp to cross 100 xp threshold
        engine.handle_event(Event::Feed { item: None });

        assert_eq!(engine.state.level, 2);
        assert_eq!(engine.state.xp, 5);
    }
}
