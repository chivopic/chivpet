use ratatui::style::Color;

use crate::model::PetStatus;

pub struct AnimationFrame {
    pub lines: &'static [&'static str],
    pub color: Color,
}

pub fn get_animation_frame(status: PetStatus, tick: usize) -> AnimationFrame {
    match status {
        PetStatus::Idle => {
            let frames: [&[&str]; 4] = [
                &[
                    " /\\_/\\ ",
                    "( o.o )",
                    " > ^ < ~",
                ],
                &[
                    " /\\_/\\ ",
                    "( o.o )",
                    " > ^ < ",
                ],
                &[
                    " /\\_/\\ ",
                    "( -.- )",
                    " > ^ < ",
                ],
                &[
                    " /\\_/\\ ",
                    "( o.o )",
                    "~ > ^ <",
                ],
            ];
            AnimationFrame {
                lines: frames[tick % frames.len()],
                color: Color::Rgb(250, 179, 135), // Catppuccin Peach
            }
        }
        PetStatus::Sleeping => {
            let frames: [&[&str]; 4] = [
                &[
                    " /\\_/\\ ",
                    "( -.- )  z",
                    " > ^ <   ",
                ],
                &[
                    " /\\_/\\ ",
                    "( -.- )   zZ",
                    " > ^ <   ",
                ],
                &[
                    " /\\_/\\ ",
                    "( -.- )    zZz",
                    " > ^ <   ",
                ],
                &[
                    " /\\_/\\ ",
                    "( u.u )  *snore*",
                    " > ^ <   ",
                ],
            ];
            AnimationFrame {
                lines: frames[tick % frames.len()],
                color: Color::Rgb(137, 180, 250), // Catppuccin Blue
            }
        }
        PetStatus::Focused => {
            let frames: [&[&str]; 3] = [
                &[
                    " /\\_/\\ ",
                    "( O.o )",
                    " > ^ < ",
                ],
                &[
                    " /\\_/\\ ",
                    "( O.O )  !",
                    " > ^ < ",
                ],
                &[
                    " /\\_/\\ ",
                    "( o.O )",
                    " > ^ < ",
                ],
            ];
            AnimationFrame {
                lines: frames[tick % frames.len()],
                color: Color::Rgb(249, 226, 175), // Catppuccin Yellow
            }
        }
        PetStatus::Cheering => {
            let frames: [&[&str]; 4] = [
                &[
                    " /\\_/\\ ",
                    "( ^.^ )/",
                    " > ^ < *purr*",
                ],
                &[
                    " /\\_/\\  ✨",
                    "\\(^.^)/ ",
                    " > ^ <  ",
                ],
                &[
                    " /\\_/\\ ",
                    "\\( ^.^)",
                    " > ^ < *sparkle*",
                ],
                &[
                    " /\\_/\\  🎉",
                    "\\(^.^)/ ",
                    " > ^ < *meow!*",
                ],
            ];
            AnimationFrame {
                lines: frames[tick % frames.len()],
                color: Color::Rgb(166, 227, 161), // Catppuccin Green
            }
        }
        PetStatus::Frustrated => {
            let frames: [&[&str]; 4] = [
                &[
                    " /\\_/\\ ",
                    "( T.T )",
                    " > v < ",
                ],
                &[
                    " /\\_/\\ ",
                    "( T.T ) 💧",
                    " > v < ",
                ],
                &[
                    " /\\_/\\ ",
                    "( ;.; )",
                    " > v < *sniff*",
                ],
                &[
                    " /\\_/\\ ",
                    "( T.T ) 💧",
                    " > v < ",
                ],
            ];
            AnimationFrame {
                lines: frames[tick % frames.len()],
                color: Color::Rgb(243, 139, 168), // Catppuccin Red
            }
        }
        PetStatus::Eating => {
            let frames: [&[&str]; 4] = [
                &[
                    " /\\_/\\ ",
                    "( >.< )~🐟",
                    " > ^ < ",
                ],
                &[
                    " /\\_/\\ ",
                    "( o.o )~🐟",
                    " > ^ < *nom*",
                ],
                &[
                    " /\\_/\\ ",
                    "( -.- )~",
                    " > ^ < *crunch*",
                ],
                &[
                    " /\\_/\\ ",
                    "( ^.^ )",
                    " > ^ < *yum!*",
                ],
            ];
            AnimationFrame {
                lines: frames[tick % frames.len()],
                color: Color::Rgb(203, 166, 247), // Catppuccin Mauve
            }
        }
        PetStatus::Hacking => {
            let frames: [&[&str]; 4] = [
                &[
                    " /\\_/\\ ",
                    "( =.= )[⌨]",
                    " /   \\  tap",
                ],
                &[
                    " /\\_/\\ ",
                    "( -.- )[⌨]",
                    " \\   /  clack",
                ],
                &[
                    " /\\_/\\ ",
                    "( O.O )[⌨]",
                    " /   \\  click",
                ],
                &[
                    " /\\_/\\ ",
                    "( =.= )[⌨]",
                    " \\   /  tap",
                ],
            ];
            AnimationFrame {
                lines: frames[tick % frames.len()],
                color: Color::Rgb(148, 226, 213), // Catppuccin Teal
            }
        }
    }
}
