use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::model::PetStatus;

pub struct StyledFrame {
    pub lines: Vec<Line<'static>>,
}

// Minimal, elegant palette: clean foreground with subtle muted tones
const FG: Color = Color::Rgb(220, 224, 232);      // Crisp, warm white
const MUTED: Color = Color::Rgb(127, 132, 156);   // Soft slate gray
const ACCENT: Color = Color::Rgb(205, 214, 244);  // Pure bright white

pub fn get_animation_frame(status: PetStatus, tick: usize) -> StyledFrame {
    match status {
        PetStatus::Idle => render_idle(tick),
        PetStatus::Hacking => render_hacking(tick),
        PetStatus::Cheering => render_cheering(tick),
        PetStatus::Sleeping => render_sleeping(tick),
        PetStatus::Frustrated => render_frustrated(tick),
        PetStatus::Eating => render_eating(tick),
        PetStatus::Focused => render_focused(tick),
    }
}

fn w(text: &'static str) -> Span<'static> {
    Span::styled(text, Style::default().fg(FG))
}

fn b(text: &'static str) -> Span<'static> {
    Span::styled(text, Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
}

fn m(text: &'static str) -> Span<'static> {
    Span::styled(text, Style::default().fg(MUTED))
}

// 12-Frame Idle: Smooth tail sine-wave (6 positions) + natural eyelid blink cycle + breathing
fn render_idle(tick: usize) -> StyledFrame {
    let f = tick % 12;
    let lines = match f {
        // Frames 0-4: Eyes open, tail swaying from right to center to left
        0 => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( • . • )   ")]),
            Line::from(vec![w("      /       \\  "), m("~")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
        1 => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( • . • )   ")]),
            Line::from(vec![w("      /       \\ "), m(") ")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
        2 => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( • . • )   ")]),
            Line::from(vec![w("      /       \\ "), m("| ")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
        3 => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( • . • )   ")]),
            Line::from(vec![w("      /       \\"), m("(  ")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
        4 => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( • . • )   ")]),
            Line::from(vec![w("      /       \\"), m("~ ")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
        // Frames 5-6: Tail starts returning, chest takes a soft gentle breath
        5 => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( • . • )   ")]),
            Line::from(vec![w("      /       \\"), m("(  ")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
        6 => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( • . • )   ")]),
            Line::from(vec![w("      /       \\ "), m("| ")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
        // Frame 7: Half-blink (eyelids lowering)
        7 => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( - . - )   ")]),
            Line::from(vec![w("      /       \\ "), m(") ")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
        // Frame 8: Full blink (eyes closed)
        8 => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( - . - )   ")]),
            Line::from(vec![w("      /       \\  "), m("~")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
        // Frame 9: Eyes reopening to half
        9 => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( • . • )   ")]),
            Line::from(vec![w("      /       \\  "), m("~")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
        // Frame 10: Tail flick with subtle ear wiggle
        10 => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( • . • )   ")]),
            Line::from(vec![w("      /       \\ "), m(") ")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
        // Frame 11: Return to base
        _ => vec![
            Line::from(vec![w("       /\\_/\\     ")]),
            Line::from(vec![w("      ( • . • )   ")]),
            Line::from(vec![w("      /       \\  "), m("~")]),
            Line::from(vec![w("     (   ( )   )  ")]),
            Line::from(vec![w("      (__) (__)   ")]),
        ],
    };

    StyledFrame { lines }
}

// 8-Frame Bongo Cat Typing: Real alternating paw dynamics
fn render_hacking(tick: usize) -> StyledFrame {
    let f = tick % 8;
    let lines = match f {
        // 0: Left paw fully down on keys, right paw high
        0 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( •̀ . •́ )   "), m("💻")]),
            Line::from(vec![w("      /   (   \\__  "), b("[===]")]),
            Line::from(vec![w("     (____)____(__)     ")]),
        ],
        // 1: Left paw lifting, right paw beginning downstroke
        1 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( •̀ . •́ )   "), m("💻")]),
            Line::from(vec![w("      /   (   ) \\  "), b("[===]")]),
            Line::from(vec![w("     (____)____(__)     ")]),
        ],
        // 2: Right paw halfway down, left paw near apex
        2 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( •̀ . •́ )   "), m("💻")]),
            Line::from(vec![w("    ___/      ) \\  "), b("[===]")]),
            Line::from(vec![w("   (__)____(____)       ")]),
        ],
        // 3: Right paw slams down on keys! Left paw at peak
        3 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( •̀ . •́ )   "), m("💻")]),
            Line::from(vec![w("    ___/   )    \\  "), b("[===]")]),
            Line::from(vec![w("   (__)____(____)       ")]),
        ],
        // 4: Right paw rebounding, left paw initiating descent
        4 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( •̀ . •́ )   "), m("💻")]),
            Line::from(vec![w("      /   )   ) \\  "), b("[===]")]),
            Line::from(vec![w("     (____)____(__)     ")]),
        ],
        // 5: Left paw descending fast
        5 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( •̀ . •́ )   "), m("💻")]),
            Line::from(vec![w("      /   (      \\ "), b("[===]")]),
            Line::from(vec![w("     (____)____(__)     ")]),
        ],
        // 6: Left paw strikes keyboard
        6 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( •̀ . •́ )   "), m("💻")]),
            Line::from(vec![w("      /   (   \\__  "), b("[===]")]),
            Line::from(vec![w("     (____)____(__)     ")]),
        ],
        // 7: Rapid double-tap flurry
        _ => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( •̀ . •́ )   "), m("💻")]),
            Line::from(vec![w("    ___/  (   \\__  "), b("[===]")]),
            Line::from(vec![w("   (____)____(__)       ")]),
        ],
    };

    StyledFrame { lines }
}

// 8-Frame Cheering: Crouch, leap, paws up flutter, soft landing
fn render_cheering(tick: usize) -> StyledFrame {
    let f = tick % 8;
    let lines = match f {
        // 0: Crouch anticipating jump
        0 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( > . < )        ")]),
            Line::from(vec![w("       / (   ) \\        ")]),
            Line::from(vec![w("      (_________)       ")]),
        ],
        // 1: Spring up, arms raising
        1 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("      /( ^ . ^ )\\       ")]),
            Line::from(vec![w("       / (   ) \\        ")]),
            Line::from(vec![w("      (__)   (__)       ")]),
        ],
        // 2: Jump peak! Arms high, sparkles
        2 => vec![
            Line::from(vec![w("        /\\_/\\     "), m("✨")]),
            Line::from(vec![w("      \\( ^ . ^ )/       ")]),
            Line::from(vec![w("       /(     )\\        ")]),
            Line::from(vec![w("      (__)   (__)       ")]),
        ],
        // 3: Float at apex, joyful expression
        3 => vec![
            Line::from(vec![m("   ✨   "), w("/\\_/\\           ")]),
            Line::from(vec![w("      \\( ^ o ^ )/       ")]),
            Line::from(vec![w("        )   (           ")]),
            Line::from(vec![w("       (__)_(__)        ")]),
        ],
        // 4: Float celebration
        4 => vec![
            Line::from(vec![w("        /\\_/\\     "), m("✨")]),
            Line::from(vec![w("      \\( ˶^.^˶ )/       ")]),
            Line::from(vec![w("        )   (           ")]),
            Line::from(vec![w("       (__)_(__)        ")]),
        ],
        // 5: Descending back down
        5 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("      /( ^ . ^ )\\       ")]),
            Line::from(vec![w("       / (   ) \\        ")]),
            Line::from(vec![w("      (__)   (__)       ")]),
        ],
        // 6: Landing cushion bounce
        6 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( ^ . ^ )        ")]),
            Line::from(vec![w("       / (   ) \\        ")]),
            Line::from(vec![w("      (_________)       ")]),
        ],
        // 7: Back on feet, happy smile
        _ => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( ^ . ^ )        ")]),
            Line::from(vec![w("       /       \\        ")]),
            Line::from(vec![w("      (__)   (__)       ")]),
        ],
    };

    StyledFrame { lines }
}

// 8-Frame Sleeping: Peaceful loaf breathing cycle + smoothly rising zZz
fn render_sleeping(tick: usize) -> StyledFrame {
    let f = tick % 8;
    let lines = match f {
        0 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( - . - )  "), m("z     ")]),
            Line::from(vec![w("      / (     ) \\       ")]),
            Line::from(vec![w("     (___________)      ")]),
        ],
        1 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( - . - )   "), m("z    ")]),
            Line::from(vec![w("      /  (   )  \\       ")]),
            Line::from(vec![w("     (___________)      ")]),
        ],
        2 => vec![
            Line::from(vec![w("        /\\_/\\     "), m("z     ")]),
            Line::from(vec![w("       ( - . - )   "), m("Z    ")]),
            Line::from(vec![w("      /  (   )  \\       ")]),
            Line::from(vec![w("     (___________)      ")]),
        ],
        3 => vec![
            Line::from(vec![w("        /\\_/\\      "), m("Z    ")]),
            Line::from(vec![w("       ( u . u )    "), m("z   ")]),
            Line::from(vec![w("      /  (   )  \\       ")]),
            Line::from(vec![w("     (___________)      ")]),
        ],
        4 => vec![
            Line::from(vec![w("        /\\_/\\       "), m("z   ")]),
            Line::from(vec![w("       ( u . u )     "), m("Z  ")]),
            Line::from(vec![w("      /  (   )  \\       ")]),
            Line::from(vec![w("     (___________)      ")]),
        ],
        5 => vec![
            Line::from(vec![w("        /\\_/\\        "), m("Z  ")]),
            Line::from(vec![w("       ( - . - )        ")]),
            Line::from(vec![w("      / (     ) \\       ")]),
            Line::from(vec![w("     (___________)      ")]),
        ],
        6 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( - . - )        ")]),
            Line::from(vec![w("      / (     ) \\       ")]),
            Line::from(vec![w("     (___________)      ")]),
        ],
        _ => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( - . - )        ")]),
            Line::from(vec![w("      / (     ) \\       ")]),
            Line::from(vec![w("     (___________)      ")]),
        ],
    };

    StyledFrame { lines }
}

// 8-Frame Frustrated: Ear droop, teardrop welling, falling, sigh
fn render_frustrated(tick: usize) -> StyledFrame {
    let f = tick % 8;
    let lines = match f {
        0 => vec![
            Line::from(vec![w("        \\___/           ")]),
            Line::from(vec![w("       ( T . T )        ")]),
            Line::from(vec![w("      / (   ) \\         ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        1 => vec![
            Line::from(vec![w("        \\___/           ")]),
            Line::from(vec![w("       ( T . T )  "), m("💧    ")]),
            Line::from(vec![w("      / (   ) \\         ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        2 => vec![
            Line::from(vec![w("        \\___/           ")]),
            Line::from(vec![w("       ( T . T )        ")]),
            Line::from(vec![w("      / (   ) \\   "), m("💧   ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        3 => vec![
            Line::from(vec![w("        \\___/           ")]),
            Line::from(vec![w("       ( T . T )        ")]),
            Line::from(vec![w("      / (   ) \\         ")]),
            Line::from(vec![w("     (  ( )  )    "), m("💧   ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        4 => vec![
            Line::from(vec![w("        \\___/           ")]),
            Line::from(vec![w("       ( ; . ; )        ")]),
            Line::from(vec![w("      / (   ) \\         ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)   "), m(".     ")]),
        ],
        5 => vec![
            Line::from(vec![w("        \\___/           ")]),
            Line::from(vec![w("       ( > . < )        ")]),
            Line::from(vec![w("      / (o o) \\         ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        6 => vec![
            Line::from(vec![w("        \\___/           ")]),
            Line::from(vec![w("       ( - . - )        ")]),
            Line::from(vec![w("      / (   ) \\         ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        _ => vec![
            Line::from(vec![w("        \\___/           ")]),
            Line::from(vec![w("       ( T . T )        ")]),
            Line::from(vec![w("      / (   ) \\         ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
    };

    StyledFrame { lines }
}

// 8-Frame Eating: Fish bite, chewing cycle, swallow, happy burp
fn render_eating(tick: usize) -> StyledFrame {
    let f = tick % 8;
    let lines = match f {
        0 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( o . o )        ")]),
            Line::from(vec![w("      / (o)o \\  "), m("><(((°>")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        1 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( > . < )        ")]),
            Line::from(vec![w("      / (o)o \\  "), m(">==<   ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        2 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( • . - )        ")]),
            Line::from(vec![w("      / (o)o \\  "), m("><>    ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        3 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( - . • )        ")]),
            Line::from(vec![w("      / (o)o \\  "), m("><>    ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        4 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( • . - )        ")]),
            Line::from(vec![w("      / (o)o \\          ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        5 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( > . < )        ")]),
            Line::from(vec![w("      /  ( )  \\         ")]),
            Line::from(vec![w("     (____)____)        ")]),
        ],
        6 => vec![
            Line::from(vec![w("        /\\_/\\     "), m("♡     ")]),
            Line::from(vec![w("       ( ˶^.^˶ )        ")]),
            Line::from(vec![w("      /  ( )  \\         ")]),
            Line::from(vec![w("     (____)____)        ")]),
        ],
        _ => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( ^ . ^ )        ")]),
            Line::from(vec![w("      /  ( )  \\         ")]),
            Line::from(vec![w("     (____)____)        ")]),
        ],
    };

    StyledFrame { lines }
}

// 6-Frame Focused: Alert perk up, eye dart, locked in
fn render_focused(tick: usize) -> StyledFrame {
    let f = tick % 6;
    let lines = match f {
        0 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( O . o )        ")]),
            Line::from(vec![w("      / (   ) \\   "), m("?     ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        1 => vec![
            Line::from(vec![w("        /\\_/\\     "), m("!     ")]),
            Line::from(vec![w("       ( O . O )        ")]),
            Line::from(vec![w("      / (   ) \\         ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        2 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( o . O )        ")]),
            Line::from(vec![w("      / (   ) \\         ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        3 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( O . O )        ")]),
            Line::from(vec![w("      / (   ) \\         ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        4 => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( • . • )        ")]),
            Line::from(vec![w("      / (   ) \\         ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
        _ => vec![
            Line::from(vec![w("        /\\_/\\           ")]),
            Line::from(vec![w("       ( O . O )        ")]),
            Line::from(vec![w("      / (   ) \\         ")]),
            Line::from(vec![w("     (  ( )  )          ")]),
            Line::from(vec![w("      (__)_(__)         ")]),
        ],
    };

    StyledFrame { lines }
}
