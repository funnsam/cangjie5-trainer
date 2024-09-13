use std::io::{self, Write};
use crossterm::{*, event::*, style::Stylize};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, terminal::Clear(terminal::ClearType::All))?;

    let mut rand = fastrand::Rng::new();
    let mut n = 0;
    let mut rescramble = |n: &mut usize| *n = rand.usize(0..cangjie_dict::CHARS.len());
    rescramble(&mut n);

    let get_char = |n: usize| &cangjie_dict::CHARS[n];

    let mut textfield = String::with_capacity(5);
    let mut ans_shown = false;
    let mut errs = Vec::new();
    let mut corr = true;

    let mut corr_c = 0_usize;
    let mut totl_c = 0_usize;

    let mut render = |n, ans_shown, corr, corr_c, totl_c, errs: &[(usize, String)], textfield: &str| {
        queue!(
            stdout,

            cursor::MoveTo(3, 1),
            terminal::Clear(terminal::ClearType::UntilNewLine),
        )?;

        if ans_shown {
            queue!(
                stdout,

                style::PrintStyledContent(
                    get_char(n).1.join(" / ").with(if corr { style::Color::Green } else { style::Color::Red }),
                ),
            )?;
        }

        queue!(
            stdout,
            cursor::MoveTo(0, 3),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            style::PrintStyledContent(format!("{corr_c} / {totl_c} ({:.01}%)", (corr_c as f64 / totl_c as f64).max(0.0) * 100.0).bold()),
        )?;

        for (eid, ec) in errs.iter() {
            let e = get_char(*eid);

            queue!(
                stdout,

                cursor::MoveDown(1),
                cursor::MoveToColumn(0),
                terminal::Clear(terminal::ClearType::UntilNewLine),
                style::Print(format!("{} ", e.0)),
                style::PrintStyledContent(ec.as_str().red()),
                cursor::MoveRight(1),
                style::PrintStyledContent(e.1.join(" / ").green()),
            )?;
        }

        queue!(
            stdout,

            cursor::MoveTo(0, 0),
            style::PrintStyledContent(get_char(n).0.to_string().bold().cyan()),

            cursor::MoveTo(3, 0),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            style::Print(textfield),
        )?;

        stdout.flush()
    };

    render(n, ans_shown, corr, corr_c, totl_c, &errs, &textfield)?;

    while let Ok(ev) = event::read() {
        match ev {
            Event::Key(KeyEvent { code: KeyCode::Esc, kind: KeyEventKind::Press, .. })
                => break,

            Event::Key(KeyEvent { code: KeyCode::Char(' '), kind: KeyEventKind::Press, .. }) => {
                if ans_shown {
                    rescramble(&mut n);
                    textfield.clear();
                } else {
                    corr = get_char(n).1.contains(&textfield.as_str());
                    if !corr {
                        errs.push((n, format!("{textfield:<5}")));
                    }

                    corr_c += corr as usize;
                    totl_c += 1;
                }
                ans_shown ^= true;
            },

            Event::Key(KeyEvent { code: KeyCode::Char(c), kind: KeyEventKind::Press, .. })
                if matches!(c, 'a'..='y') && textfield.len() < 5 && !ans_shown
                    => textfield.push(c),
            Event::Key(KeyEvent { code: KeyCode::Backspace, kind: KeyEventKind::Press, .. })
                if !ans_shown
                    => { textfield.pop(); },
            _ => {},
        }

        errs.drain(..errs.len().saturating_sub(terminal::size()?.1 as usize - 4));

        render(n, ans_shown, corr, corr_c, totl_c, &errs, &textfield)?;
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    Ok(())
}
