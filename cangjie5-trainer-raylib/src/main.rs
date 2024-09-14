use raylib::prelude::*;
use indexmap::IndexSet;

mod args;
mod macchiato;

static FONTS: &[&str] = &[
    "font/FSung-p.ttf",
    "font/SourceCodePro.ttf",
    "font/FSung-2.ttf",
    "font/FSung-3.ttf",
];
const RAND_BUFFER: usize = 32;
const PLANE_MAX_CHARS: usize = 64;

fn main() {
    let args = <args::Args as clap::Parser>::parse();

    let (mut rl, thread) = raylib::init()
        .resizable()
        .msaa_4x()
        .vsync()
        .log_level(consts::TraceLogLevel::LOG_ERROR)
        .title("Cangjie 5 trainer")
        .build();
    rl.set_exit_key(None);


    let mut fonts = [
        rl.load_font_ex(&thread, "font/FSung-p.ttf", 48, Some("a")).unwrap(),
        rl.load_font_ex(&thread, "font/SourceCodePro.ttf", 24, None).unwrap(),
        rl.load_font_ex(&thread, "font/FSung-2.ttf", 48, Some("a")).unwrap(),
        rl.load_font_ex(&thread, "font/FSung-3.ttf", 48, Some("a")).unwrap(),
    ];
    let mut loaded_chars: [IndexSet<char>; 4] = [
        IndexSet::new(),
        IndexSet::new(),
        IndexSet::new(),
        IndexSet::new(),
    ];

    macro_rules! font {
        (ui) => { fonts[1] };
        ($c: expr) => { fonts[plane!($c)] };
    }

    let mut rand = fastrand::Rng::new();
    let mut char_idxs: [usize; RAND_BUFFER] = [0; RAND_BUFFER];
    let mut char_at = usize::MAX - 1;

    let mut textfield = String::with_capacity(5);
    let mut ans_shown = false;
    let mut errs = Vec::new();
    let mut corr = true;

    let mut corr_c = 0_usize;
    let mut totl_c = 0_usize;

    let get_char = |n: usize| &cangjie_dict::CHARS[n];

    macro_rules! reload_plane {
        ($plane: expr) => {{
            let mut chars = loaded_chars[$plane].iter().fold(String::with_capacity(loaded_chars[$plane].len()), |a, c| a + c.to_string().as_str());
            for (eid, _) in errs.iter() {
                if plane!(*eid) == $plane { chars.push(get_char(*eid).0); }
            }

            fonts[$plane] = rl.load_font_ex(&thread, FONTS[$plane], 48, Some(if !chars.is_empty() { &chars } else { "a" })).unwrap();
            println!("\x1b[1;34mInfo:\x1b[0m reloaded plane {} with {} characters", $plane, chars.chars().count());
        }};
    }

    macro_rules! n { () => { char_idxs[char_at] }; }
    macro_rules! plane { ($c: expr) => { get_char($c).0 as u32 as usize >> 16 }; }

    macro_rules! rescramble {
        () => {{
            if char_at + 1 >= RAND_BUFFER {
                char_at = 0;
                let mut updated = [false; 4];

                for c in char_idxs.iter_mut() {
                    *c = rand.usize(0..cangjie_dict::CHARS.len());

                    let plane = &mut loaded_chars[plane!(*c)];
                    plane.insert(get_char(*c).0);
                    plane.drain(0..plane.len().saturating_sub(PLANE_MAX_CHARS));

                    updated[plane!(*c)] = true;
                }

                updated.into_iter().enumerate().filter(|(_, u)| *u).for_each(|(p, _)| reload_plane!(p));
            } else {
                char_at += 1;
            }
        }};
    }
    rescramble!();

    while !rl.window_should_close() {
        // --- logic ---
        let ok = |textfield: &str, ans_shown: bool| ((textfield.len() < 5 && !args.quick) || (textfield.len() < 2 && args.quick)) && !ans_shown;

        match rl.get_key_pressed() {
            Some(KeyboardKey::KEY_SPACE) => {
                if ans_shown {
                    rescramble!();
                    textfield.clear();
                } else {
                    corr = if !args.quick {
                        get_char(n!()).1.contains(&textfield.as_str())
                    } else {
                        get_char(n!()).1.iter().any(|s| s == &textfield || (textfield.len() != 0 && s.as_bytes()[0] == textfield.as_bytes()[0] && s.as_bytes().last() == textfield.as_bytes().last()))
                    };

                    if !corr {
                        errs.push((n!(), format!("{textfield:<5}")));
                        errs.drain(..errs.len().saturating_sub(32));
                    }

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

        d.draw_text_ex(&font!(n!()), &get_char(n!()).0.to_string(), Vector2::new(12.0, 12.0), 48.0, 0.0, macchiato::TEXT);
        d.draw_text_ex(&font!(ui), &textfield, Vector2::new(72.0, 30.0), 24.0, 0.0, macchiato::SUBTEXT1);

        if ans_shown {
            d.draw_text_ex(
                &font!(ui),
                &get_char(n!()).1.join(" / "),
                Vector2::new(72.0, 58.0),
                24.0,
                0.0,
                if corr { macchiato::GREEN } else { macchiato::RED },
            );
        }

        d.draw_text_ex(
            &font!(ui),
            &format!("{corr_c} / {totl_c} ({:.01}%)", (corr_c as f64 / totl_c as f64).min(1.0) * 100.0),
            Vector2::new(12.0, 96.0),
            24.0,
            0.0,
            macchiato::TEXT,
        );

        for (i, (eid, ec)) in errs.iter().rev().enumerate() {
            let y = 120.0 + i as f32 * 24.0;
            let e = get_char(*eid);

            d.draw_text_ex(&font!(*eid), &e.0.to_string(), Vector2::new(12.0, y), 24.0, 0.0, macchiato::TEXT);
            d.draw_text_ex(&font!(ui), &ec, Vector2::new(48.0, y), 24.0, 0.0, macchiato::RED);
            d.draw_text_ex(&font!(ui), &e.1.join(" / "), Vector2::new(120.0, y), 24.0, 0.0, macchiato::GREEN);
        }
    }
}
