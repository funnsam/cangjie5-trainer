use raylib::prelude::*;

mod args;
mod macchiato;

// https://github.com/catppuccin/catppuccin#-palette

fn main() {
    let args = <args::Args as clap::Parser>::parse();

    let (mut rl, thread) = raylib::init()
        .resizable()
        .msaa_4x()
        .vsync()
        .title("Cangjie 5 trainer")
        .build();
    rl.set_exit_key(None);

    // TODO: use fsung
    let big_chi_font = rl.load_font_ex(&thread, "noto_sans_hk.ttf", 48, Some(cangjie_dict::ALL)).unwrap();
    let ui_font = rl.load_font_ex(&thread, "font/FSung-p.ttf", 28, None).unwrap();

    let mut rand = fastrand::Rng::new();
    let mut n = 0;
    let mut rescramble = |n: &mut usize| *n = rand.usize(0..cangjie_dict::CHARS.len());
    rescramble(&mut n);

    let get_char = |n| &cangjie_dict::CHARS[n];

    let mut textfield = String::with_capacity(5);
    let mut ans_shown = false;
    let mut errs = String::new();
    let mut corr = true;

    let mut corr_c = 0_usize;
    let mut totl_c = 0_usize;

    while !rl.window_should_close() {
        // --- logic ---
        let ok = |textfield: &str, ans_shown: bool| ((textfield.len() < 5 && !args.quick) || (textfield.len() < 2 && args.quick)) && !ans_shown;

        match rl.get_key_pressed() {
            Some(KeyboardKey::KEY_SPACE) => {
                if ans_shown {
                    rescramble(&mut n);
                    textfield.clear();
                } else {
                    corr = if !args.quick {
                        get_char(n).1.contains(&textfield.as_str())
                    } else {
                        get_char(n).1.iter().any(|s| s == &textfield || (textfield.len() != 0 && s.as_bytes()[0] == textfield.as_bytes()[0] && s.as_bytes().last() == textfield.as_bytes().last()))
                    };

                    if !corr { errs.push(get_char(n).0); }

                    corr_c += corr as usize;
                    totl_c += 1;
                }
                ans_shown ^= true;
            },

            Some(KeyboardKey::KEY_A) if ok(&textfield, ans_shown) => textfield.push('a'),
            Some(KeyboardKey::KEY_B) if ok(&textfield, ans_shown) => textfield.push('b'),
            Some(KeyboardKey::KEY_C) if ok(&textfield, ans_shown) => textfield.push('c'),
            Some(KeyboardKey::KEY_D) if ok(&textfield, ans_shown) => textfield.push('d'),
            Some(KeyboardKey::KEY_E) if ok(&textfield, ans_shown) => textfield.push('e'),
            Some(KeyboardKey::KEY_F) if ok(&textfield, ans_shown) => textfield.push('f'),
            Some(KeyboardKey::KEY_G) if ok(&textfield, ans_shown) => textfield.push('g'),
            Some(KeyboardKey::KEY_H) if ok(&textfield, ans_shown) => textfield.push('h'),
            Some(KeyboardKey::KEY_I) if ok(&textfield, ans_shown) => textfield.push('i'),
            Some(KeyboardKey::KEY_J) if ok(&textfield, ans_shown) => textfield.push('j'),
            Some(KeyboardKey::KEY_K) if ok(&textfield, ans_shown) => textfield.push('k'),
            Some(KeyboardKey::KEY_L) if ok(&textfield, ans_shown) => textfield.push('l'),
            Some(KeyboardKey::KEY_M) if ok(&textfield, ans_shown) => textfield.push('m'),
            Some(KeyboardKey::KEY_N) if ok(&textfield, ans_shown) => textfield.push('n'),
            Some(KeyboardKey::KEY_O) if ok(&textfield, ans_shown) => textfield.push('o'),
            Some(KeyboardKey::KEY_P) if ok(&textfield, ans_shown) => textfield.push('p'),
            Some(KeyboardKey::KEY_Q) if ok(&textfield, ans_shown) => textfield.push('q'),
            Some(KeyboardKey::KEY_R) if ok(&textfield, ans_shown) => textfield.push('r'),
            Some(KeyboardKey::KEY_S) if ok(&textfield, ans_shown) => textfield.push('s'),
            Some(KeyboardKey::KEY_T) if ok(&textfield, ans_shown) => textfield.push('t'),
            Some(KeyboardKey::KEY_U) if ok(&textfield, ans_shown) => textfield.push('u'),
            Some(KeyboardKey::KEY_V) if ok(&textfield, ans_shown) => textfield.push('v'),
            Some(KeyboardKey::KEY_W) if ok(&textfield, ans_shown) => textfield.push('w'),
            Some(KeyboardKey::KEY_X) if ok(&textfield, ans_shown) => textfield.push('x'),
            Some(KeyboardKey::KEY_Y) if ok(&textfield, ans_shown) => textfield.push('y'),

            Some(KeyboardKey::KEY_BACKSPACE) if !ans_shown => { textfield.pop(); },
            _ => {},
        }

        // --- render ---
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(macchiato::BASE);

        d.draw_text_ex(&big_chi_font, &get_char(n).0.to_string(), Vector2::new(12.0, 12.0), 48.0, 0.0, macchiato::TEXT);
        d.draw_text_ex(&ui_font, &textfield, Vector2::new(72.0, 30.0), 28.0, 0.0, macchiato::SUBTEXT1);

        if ans_shown {
            d.draw_text_ex(
                &ui_font,
                &get_char(n).1.join(" / "),
                Vector2::new(72.0, 58.0),
                28.0,
                0.0,
                if corr { macchiato::GREEN } else { MACCHIATO_RED },
            );
        }

        d.draw_text_ex(&big_chi_font, &errs, Vector2::new(12.0, 96.0), 24.0, 0.0, macchiato::SUBTEXT0);

        d.draw_fps(0, 0);
    }
}
