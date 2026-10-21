use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::model::PetStatus;

pub struct StyledFrame {
    pub lines: Vec<Line<'static>>,
    pub primary_color: Color,
}

// Catppuccin Mocha Colors
const PEACH: Color = Color::Rgb(250, 179, 135);
const BLUE: Color = Color::Rgb(137, 180, 250);
const YELLOW: Color = Color::Rgb(249, 226, 175);
const GREEN: Color = Color::Rgb(166, 227, 161);
const RED: Color = Color::Rgb(243, 139, 168);
const PINK: Color = Color::Rgb(245, 194, 231);
const MAUVE: Color = Color::Rgb(203, 166, 247);
const TEAL: Color = Color::Rgb(148, 226, 213);
const SAPPHIRE: Color = Color::Rgb(116, 199, 236);
const GRAY: Color = Color::Rgb(108, 112, 134);

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

fn s(text: &'static str, color: Color, bold: bool) -> Span<'static> {
    let mut style = Style::default().fg(color);
    if bold {
        style = style.add_modifier(Modifier::BOLD);
    }
    Span::styled(text, style)
}

fn render_idle(tick: usize) -> StyledFrame {
    let frame_idx = tick % 6;
    let lines = match frame_idx {
        0 => vec![
            Line::from(vec![s("      /\\_/\\     ", PEACH, true)]),
            Line::from(vec![s("     ( o.o )    ", PEACH, true)]),
            Line::from(vec![s("     /     \\   ", PEACH, true), s("~", PINK, true)]),
            Line::from(vec![s("    (  ( )  )   ", PEACH, true)]),
            Line::from(vec![s("     (__)_(__)  ", PEACH, true)]),
        ],
        1 => vec![
            Line::from(vec![s("      /\\_/\\     ", PEACH, true)]),
            Line::from(vec![s("     ( o.o )    ", PEACH, true)]),
            Line::from(vec![s("     /     \\   ", PEACH, true), s("|", PINK, true)]),
            Line::from(vec![s("    (  ( )  )   ", PEACH, true)]),
            Line::from(vec![s("     (__)_(__)  ", PEACH, true)]),
        ],
        2 => vec![
            Line::from(vec![s("      /\\_/\\     ", PEACH, true)]),
            Line::from(vec![s("     ( o.o )    ", PEACH, true)]),
            Line::from(vec![s("     /     \\   ", PEACH, true), s("/", PINK, true)]),
            Line::from(vec![s("    (  ( )  )   ", PEACH, true)]),
            Line::from(vec![s("     (__)_(__)  ", PEACH, true)]),
        ],
        3 => vec![
            Line::from(vec![s("      /\\_/\\     ", PEACH, true)]),
            Line::from(vec![s("     ( -.- )    ", PEACH, true)]),
            Line::from(vec![s("     /     \\   ", PEACH, true), s("~", PINK, true)]),
            Line::from(vec![s("    (  ( )  )   ", PEACH, true)]),
            Line::from(vec![s("     (__)_(__)  ", PEACH, true)]),
        ],
        4 => vec![
            Line::from(vec![s("      /\\_/\\   ", PEACH, true), s("♡", PINK, true)]),
            Line::from(vec![s("     ( o.o )    ", PEACH, true)]),
            Line::from(vec![s("     /     \\   ", PEACH, true), s("~", PINK, true)]),
            Line::from(vec![s("    (  ( )  )   ", PEACH, true)]),
            Line::from(vec![s("     (__)_(__)  ", PEACH, true)]),
        ],
        _ => vec![
            Line::from(vec![s("      /\\_/\\     ", PEACH, true)]),
            Line::from(vec![s("     ( ^.^ )    ", PEACH, true)]),
            Line::from(vec![s("     /     \\   ", PEACH, true), s("~", PINK, true)]),
            Line::from(vec![s("    (  ( )  )   ", PEACH, true)]),
            Line::from(vec![s("     (__)_(__)  ", PEACH, true)]),
        ],
    };

    StyledFrame {
        lines,
        primary_color: PEACH,
    }
}

fn render_hacking(tick: usize) -> StyledFrame {
    let frame_idx = tick % 4;
    let lines = match frame_idx {
        0 => vec![
            Line::from(vec![s("      /\\_/\\           ", PEACH, true)]),
            Line::from(vec![s("     ( •̀ ω •́ )   ", PEACH, true), s("💻", TEAL, true)]),
            Line::from(vec![
                s("    /  (   \\__  ", PEACH, true),
                s("[===] ", MAUVE, true),
                s("tap!", GREEN, true),
            ]),
            Line::from(vec![s("   (____)____(__)     ", PEACH, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
        1 => vec![
            Line::from(vec![s("      /\\_/\\   ", PEACH, true), s("⚡      ", YELLOW, true)]),
            Line::from(vec![s("     ( •̀ ω •́ )   ", PEACH, true), s("💻", TEAL, true)]),
            Line::from(vec![
                s("    /  (   ) \\  ", PEACH, true),
                s("[===] ", MAUVE, true),
                s("...", GRAY, true),
            ]),
            Line::from(vec![s("   (____)____(__)     ", PEACH, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
        2 => vec![
            Line::from(vec![s("      /\\_/\\           ", PEACH, true)]),
            Line::from(vec![s("     ( •̀ ω •́ )   ", PEACH, true), s("💻", TEAL, true)]),
            Line::from(vec![
                s("   ___/   )  \\  ", PEACH, true),
                s("[===] ", MAUVE, true),
                s("clack!", GREEN, true),
            ]),
            Line::from(vec![s("  (__)____(____)      ", PEACH, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
        _ => vec![
            Line::from(vec![s("      /\\_/\\   ", PEACH, true), s("⚡      ", YELLOW, true)]),
            Line::from(vec![s("     ( ˶•̀ ᵥ •́˶)  ", PEACH, true), s("💻", TEAL, true)]),
            Line::from(vec![
                s("   ___/   \\___  ", PEACH, true),
                s("[===] ", MAUVE, true),
                s("click!", TEAL, true),
            ]),
            Line::from(vec![s("  (____)____(____)    ", PEACH, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
    };

    StyledFrame {
        lines,
        primary_color: TEAL,
    }
}

fn render_cheering(tick: usize) -> StyledFrame {
    let frame_idx = tick % 4;
    let lines = match frame_idx {
        0 => vec![
            Line::from(vec![s("      /\\_/\\   ", GREEN, true), s("✨", YELLOW, true)]),
            Line::from(vec![s("    \\( ^.^ )/  ", GREEN, true), s("🌟", YELLOW, true)]),
            Line::from(vec![s("     /(   )\\   ", GREEN, true), s("*purr*", PINK, true)]),
            Line::from(vec![s("    (__)_(__)         ", GREEN, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
        1 => vec![
            Line::from(vec![s("      /\\_/\\   ", GREEN, true), s("🎉", YELLOW, true)]),
            Line::from(vec![s("    \\( >.< )/ ", GREEN, true), s("✨", YELLOW, true)]),
            Line::from(vec![s("      )   (    ", GREEN, true), s("*meow!*", PINK, true)]),
            Line::from(vec![s("     (__)_(__)        ", GREEN, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
        2 => vec![
            Line::from(vec![s("      /\\_/\\   ", GREEN, true), s("🌟", YELLOW, true)]),
            Line::from(vec![s("     \\(^o^)/   ", GREEN, true), s("✨", YELLOW, true)]),
            Line::from(vec![s("     /(   )\\   ", GREEN, true), s("*hooray!*", PINK, true)]),
            Line::from(vec![s("    (__)_(__)         ", GREEN, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
        _ => vec![
            Line::from(vec![s("      /\\_/\\   ", GREEN, true), s("♡", PINK, true)]),
            Line::from(vec![s("    \\( ˶^.^˶)/ ", GREEN, true), s("🎉", YELLOW, true)]),
            Line::from(vec![s("      )   (    ", GREEN, true), s("*purr*", PINK, true)]),
            Line::from(vec![s("     (__)_(__)        ", GREEN, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
    };

    StyledFrame {
        lines,
        primary_color: GREEN,
    }
}

fn render_sleeping(tick: usize) -> StyledFrame {
    let frame_idx = tick % 4;
    let lines = match frame_idx {
        0 => vec![
            Line::from(vec![s("      /\\_/\\           ", BLUE, true)]),
            Line::from(vec![s("     ( -.- )  ", BLUE, true), s("z", SAPPHIRE, true)]),
            Line::from(vec![s("    / (   ) \\         ", BLUE, true)]),
            Line::from(vec![s("   (____)____)        ", BLUE, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
        1 => vec![
            Line::from(vec![s("      /\\_/\\           ", BLUE, true)]),
            Line::from(vec![s("     ( -.- )   ", BLUE, true), s("zZ", SAPPHIRE, true)]),
            Line::from(vec![s("    / (   ) \\         ", BLUE, true)]),
            Line::from(vec![s("   (_____)____)       ", BLUE, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
        2 => vec![
            Line::from(vec![s("      /\\_/\\           ", BLUE, true)]),
            Line::from(vec![s("     ( u.u )    ", BLUE, true), s("zZz", SAPPHIRE, true)]),
            Line::from(vec![s("    / (   ) \\         ", BLUE, true)]),
            Line::from(vec![s("   (____)____)        ", BLUE, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
        _ => vec![
            Line::from(vec![s("      /\\_/\\   ", BLUE, true), s("*snore*", TEAL, true)]),
            Line::from(vec![s("     ( u.u )     ", BLUE, true), s("ZzZ", SAPPHIRE, true)]),
            Line::from(vec![s("    / (   ) \\         ", BLUE, true)]),
            Line::from(vec![s("   (_____)____)       ", BLUE, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
    };

    StyledFrame {
        lines,
        primary_color: BLUE,
    }
}

fn render_frustrated(tick: usize) -> StyledFrame {
    let frame_idx = tick % 4;
    let lines = match frame_idx {
        0 => vec![
            Line::from(vec![s("      \\___/           ", RED, true)]),
            Line::from(vec![s("     ( T.T )          ", RED, true)]),
            Line::from(vec![s("    / (   ) \\         ", RED, true)]),
            Line::from(vec![s("   (  ( )  )  ", RED, true), s("💧", SAPPHIRE, true)]),
            Line::from(vec![s("    (__)_(__)         ", RED, true)]),
        ],
        1 => vec![
            Line::from(vec![s("      \\___/           ", RED, true)]),
            Line::from(vec![s("     ( T.T )          ", RED, true)]),
            Line::from(vec![s("    / (   ) \\  ", RED, true), s("💧", SAPPHIRE, true)]),
            Line::from(vec![s("   (  ( )  )          ", RED, true)]),
            Line::from(vec![s("    (__)_(__) ", RED, true), s("💧", SAPPHIRE, true)]),
        ],
        2 => vec![
            Line::from(vec![s("      \\___/           ", RED, true)]),
            Line::from(vec![s("     ( ;.; )  ", RED, true), s("*sniff*", GRAY, true)]),
            Line::from(vec![s("    / (   ) \\         ", RED, true)]),
            Line::from(vec![s("   (  ( )  )  ", RED, true), s("💧", SAPPHIRE, true)]),
            Line::from(vec![s("    (__)_(__)         ", RED, true)]),
        ],
        _ => vec![
            Line::from(vec![s("      \\___/           ", RED, true)]),
            Line::from(vec![s("     ( >.< )          ", RED, true)]),
            Line::from(vec![s("    / (o o) \\         ", RED, true)]),
            Line::from(vec![s("   (  ( )  )          ", RED, true)]),
            Line::from(vec![s("    (__)_(__)         ", RED, true)]),
        ],
    };

    StyledFrame {
        lines,
        primary_color: RED,
    }
}

fn render_eating(tick: usize) -> StyledFrame {
    let frame_idx = tick % 4;
    let lines = match frame_idx {
        0 => vec![
            Line::from(vec![s("      /\\_/\\           ", PEACH, true)]),
            Line::from(vec![s("     ( o.o )          ", PEACH, true)]),
            Line::from(vec![
                s("    / (o)o \\  ", PEACH, true),
                s("><(((°>", SAPPHIRE, true),
            ]),
            Line::from(vec![s("   (  ( )  )          ", PEACH, true)]),
            Line::from(vec![s("    (__)_(__)         ", PEACH, true)]),
        ],
        1 => vec![
            Line::from(vec![s("      /\\_/\\           ", PEACH, true)]),
            Line::from(vec![s("     ( >.< )  ", PEACH, true), s("*nom*", PINK, true)]),
            Line::from(vec![
                s("    / (o)o \\  ", PEACH, true),
                s(">==<", SAPPHIRE, true),
            ]),
            Line::from(vec![s("   (  ( )  )          ", PEACH, true)]),
            Line::from(vec![s("    (__)_(__)         ", PEACH, true)]),
        ],
        2 => vec![
            Line::from(vec![s("      /\\_/\\           ", PEACH, true)]),
            Line::from(vec![
                s("     ( -.- )  ", PEACH, true),
                s("*crunch*", YELLOW, true),
            ]),
            Line::from(vec![
                s("    / (o)o \\  ", PEACH, true),
                s("><>", SAPPHIRE, true),
            ]),
            Line::from(vec![s("   (  ( )  )          ", PEACH, true)]),
            Line::from(vec![s("    (__)_(__)         ", PEACH, true)]),
        ],
        _ => vec![
            Line::from(vec![s("      /\\_/\\   ", PEACH, true), s("*yum!*", PINK, true)]),
            Line::from(vec![s("     ( ^.^ )  ", PEACH, true), s("♡", PINK, true)]),
            Line::from(vec![s("    /  ( )  \\         ", PEACH, true)]),
            Line::from(vec![s("   (____)____)        ", PEACH, true)]),
            Line::from(vec![s("                      ", GRAY, false)]),
        ],
    };

    StyledFrame {
        lines,
        primary_color: PINK,
    }
}

fn render_focused(tick: usize) -> StyledFrame {
    let frame_idx = tick % 3;
    let lines = match frame_idx {
        0 => vec![
            Line::from(vec![s("      /\\_/\\           ", YELLOW, true)]),
            Line::from(vec![s("     ( O.o )          ", YELLOW, true)]),
            Line::from(vec![s("    / (   ) \\  ", YELLOW, true), s("?", PEACH, true)]),
            Line::from(vec![s("   (  ( )  )          ", YELLOW, true)]),
            Line::from(vec![s("    (__)_(__)         ", YELLOW, true)]),
        ],
        1 => vec![
            Line::from(vec![s("      /\\_/\\   ", YELLOW, true), s("!", RED, true)]),
            Line::from(vec![s("     ( O.O )          ", YELLOW, true)]),
            Line::from(vec![s("    / (   ) \\         ", YELLOW, true)]),
            Line::from(vec![s("   (  ( )  )          ", YELLOW, true)]),
            Line::from(vec![s("    (__)_(__)         ", YELLOW, true)]),
        ],
        _ => vec![
            Line::from(vec![s("      /\\_/\\           ", YELLOW, true)]),
            Line::from(vec![s("     ( o.O )          ", YELLOW, true)]),
            Line::from(vec![s("    / (   ) \\  ", YELLOW, true), s("...", GRAY, true)]),
            Line::from(vec![s("   (  ( )  )          ", YELLOW, true)]),
            Line::from(vec![s("    (__)_(__)         ", YELLOW, true)]),
        ],
    };

    StyledFrame {
        lines,
        primary_color: YELLOW,
    }
}
