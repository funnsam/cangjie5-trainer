use raylib::prelude::*;

// https://github.com/catppuccin/catppuccin#-palette
pub const MACCHIATO_ROSEWATER: Color = Color::new(244, 219, 214, 255);
pub const MACCHIATO_FLAMINGO: Color = Color::new(240, 198, 198, 255);
pub const MACCHIATO_PINK: Color = Color::new(245, 189, 230, 255);
pub const MACCHIATO_MAUVE: Color = Color::new(198, 160, 246, 255);
pub const MACCHIATO_RED: Color = Color::new(237, 135, 150, 255);
pub const MACCHIATO_MAROON: Color = Color::new(238, 153, 160, 255);
pub const MACCHIATO_PEACH: Color = Color::new(245, 169, 127, 255);
pub const MACCHIATO_YELLOW: Color = Color::new(238, 212, 159, 255);
pub const MACCHIATO_GREEN: Color = Color::new(166, 218, 149, 255);
pub const MACCHIATO_TEAL: Color = Color::new(139, 213, 202, 255);
pub const MACCHIATO_SKY: Color = Color::new(145, 215, 227, 255);
pub const MACCHIATO_SAPPHIRE: Color = Color::new(125, 196, 228, 255);
pub const MACCHIATO_BLUE: Color = Color::new(138, 173, 244, 255);
pub const MACCHIATO_LAVENDER: Color = Color::new(183, 189, 248, 255);
pub const MACCHIATO_TEXT: Color = Color::new(202, 211, 245, 255);
pub const MACCHIATO_SUBTEXT1: Color = Color::new(184, 192, 224, 255);
pub const MACCHIATO_SUBTEXT0: Color = Color::new(165, 173, 203, 255);
pub const MACCHIATO_OVERLAY2: Color = Color::new(147, 154, 183, 255);
pub const MACCHIATO_OVERLAY1: Color = Color::new(128, 135, 162, 255);
pub const MACCHIATO_OVERLAY0: Color = Color::new(110, 115, 141, 255);
pub const MACCHIATO_SURFACE2: Color = Color::new(91, 96, 120, 255);
pub const MACCHIATO_SURFACE1: Color = Color::new(73, 77, 100, 255);
pub const MACCHIATO_SURFACE0: Color = Color::new(54, 58, 79, 255);
pub const MACCHIATO_BASE: Color = Color::new(36, 39, 58, 255);
pub const MACCHIATO_MANTLE: Color = Color::new(30, 32, 48, 255);
pub const MACCHIATO_CRUST: Color = Color::new(24, 25, 38, 255);

fn main() {
    let (mut rl, thread) = raylib::init()
        .resizable()
        .msaa_4x()
        .vsync()
        .title("Cangjie 5 trainer")
        .build();
    rl.set_exit_key(None);

    let big_chi_font = rl.load_font_ex(&thread, "noto_sans_hk.ttf", 48, Some(cangjie_dict::ALL)).unwrap();
    let ui_font = rl.load_font_ex(&thread, "noto_sans_hk.ttf", 28, None).unwrap();

    let mut rand = fastrand::Rng::new();
    let mut n = 0;
    let mut rescramble = |n: &mut usize| *n = rand.usize(0..cangjie_dict::CHARS.len());
    rescramble(&mut n);

    let get_char = |n| &cangjie_dict::CHARS[n];

    let mut textfield = String::with_capacity(5);
    let mut ans_shown = false;
    let mut errs = String::new();
    let mut corr = true;

    while !rl.window_should_close() {
        // --- logic ---
        match rl.get_key_pressed() {
            Some(KeyboardKey::KEY_SPACE) => {
                if ans_shown {
                    rescramble(&mut n);
                    textfield.clear();
                } else {
                    corr = get_char(n).1.contains(&textfield.as_str());
                    if !corr { errs.push(get_char(n).0); }
                }
                ans_shown ^= true;
            },

            Some(KeyboardKey::KEY_A) if textfield.len() < 5 && !ans_shown => textfield.push('a'),
            Some(KeyboardKey::KEY_B) if textfield.len() < 5 && !ans_shown => textfield.push('b'),
            Some(KeyboardKey::KEY_C) if textfield.len() < 5 && !ans_shown => textfield.push('c'),
            Some(KeyboardKey::KEY_D) if textfield.len() < 5 && !ans_shown => textfield.push('d'),
            Some(KeyboardKey::KEY_E) if textfield.len() < 5 && !ans_shown => textfield.push('e'),
            Some(KeyboardKey::KEY_F) if textfield.len() < 5 && !ans_shown => textfield.push('f'),
            Some(KeyboardKey::KEY_G) if textfield.len() < 5 && !ans_shown => textfield.push('g'),
            Some(KeyboardKey::KEY_H) if textfield.len() < 5 && !ans_shown => textfield.push('h'),
            Some(KeyboardKey::KEY_I) if textfield.len() < 5 && !ans_shown => textfield.push('i'),
            Some(KeyboardKey::KEY_J) if textfield.len() < 5 && !ans_shown => textfield.push('j'),
            Some(KeyboardKey::KEY_K) if textfield.len() < 5 && !ans_shown => textfield.push('k'),
            Some(KeyboardKey::KEY_L) if textfield.len() < 5 && !ans_shown => textfield.push('l'),
            Some(KeyboardKey::KEY_M) if textfield.len() < 5 && !ans_shown => textfield.push('m'),
            Some(KeyboardKey::KEY_N) if textfield.len() < 5 && !ans_shown => textfield.push('n'),
            Some(KeyboardKey::KEY_O) if textfield.len() < 5 && !ans_shown => textfield.push('o'),
            Some(KeyboardKey::KEY_P) if textfield.len() < 5 && !ans_shown => textfield.push('p'),
            Some(KeyboardKey::KEY_Q) if textfield.len() < 5 && !ans_shown => textfield.push('q'),
            Some(KeyboardKey::KEY_R) if textfield.len() < 5 && !ans_shown => textfield.push('r'),
            Some(KeyboardKey::KEY_S) if textfield.len() < 5 && !ans_shown => textfield.push('s'),
            Some(KeyboardKey::KEY_T) if textfield.len() < 5 && !ans_shown => textfield.push('t'),
            Some(KeyboardKey::KEY_U) if textfield.len() < 5 && !ans_shown => textfield.push('u'),
            Some(KeyboardKey::KEY_V) if textfield.len() < 5 && !ans_shown => textfield.push('v'),
            Some(KeyboardKey::KEY_W) if textfield.len() < 5 && !ans_shown => textfield.push('w'),
            Some(KeyboardKey::KEY_X) if textfield.len() < 5 && !ans_shown => textfield.push('x'),
            Some(KeyboardKey::KEY_Y) if textfield.len() < 5 && !ans_shown => textfield.push('y'),

            Some(KeyboardKey::KEY_BACKSPACE) => { textfield.pop(); },
            _ => {},
        }

        // --- render ---
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(MACCHIATO_BASE);

        d.draw_text_ex(&big_chi_font, &get_char(n).0.to_string(), Vector2::new(12.0, 12.0), 48.0, 0.0, MACCHIATO_TEXT);
        d.draw_text_ex(&ui_font, &textfield, Vector2::new(72.0, 30.0), 28.0, 0.0, MACCHIATO_SUBTEXT1);

        if ans_shown {
            d.draw_text_ex(
                &ui_font,
                &get_char(n).1.join(" / "),
                Vector2::new(72.0, 58.0),
                28.0,
                0.0,
                if corr { MACCHIATO_GREEN } else { MACCHIATO_RED },
            );
        }

        d.draw_text_ex(&big_chi_font, &errs, Vector2::new(12.0, 96.0), 24.0, 0.0, MACCHIATO_SUBTEXT0);

        d.draw_fps(0, 0);
    }
}
