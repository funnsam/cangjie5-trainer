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
    // let mut errs = String::new();
    let mut corr = true;

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        style::PrintStyledContent(get_char(n).0.to_string().bold().cyan()),
    )?;

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
                    // if !corr { errs.push(get_char(n).0); }
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

        // --- render ---
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

            cursor::MoveTo(0, 0),
            style::PrintStyledContent(get_char(n).0.to_string().bold().cyan()),

            cursor::MoveTo(3, 0),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            style::Print(&textfield),
        )?;

        stdout.flush()?;
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen)?;
    Ok(())
}
