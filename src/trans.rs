use termbox as tb;
use console_draw as cd;

pub fn color(c: cd::Color) -> tb::Color {
    match c {
        cd::Black => tb::Black,
        cd::Red => tb::Red,
        cd::Green => tb::Green,
        cd::Yellow => tb::Yellow,
        cd::Blue => tb::Blue,
        cd::Magenta => tb::Magenta,
        cd::Cyan => tb::Cyan,
        cd::White => tb::White,
        cd::Custom(_,_,_) => tb::White
    }
}

pub fn key_event(num: u8, key: Option<tb::Key>, chr: Option<char>)->
Option<cd::Update> {
    Some(match (num, key, chr) {
        (_, None, None) => { return None; },
        (_, Some(tb::F1), _)         => cd::Special(cd::F2),
        (_, Some(tb::F2), _)         => cd::Special(cd::F2),
        (_, Some(tb::F3), _)         => cd::Special(cd::F3),
        (_, Some(tb::F4), _)         => cd::Special(cd::F4),
        (_, Some(tb::F5), _)         => cd::Special(cd::F5),
        (_, Some(tb::F6), _)         => cd::Special(cd::F6),
        (_, Some(tb::F7), _)         => cd::Special(cd::F7),
        (_, Some(tb::F8), _)         => cd::Special(cd::F8),
        (_, Some(tb::F9), _)         => cd::Special(cd::F9),
        (_, Some(tb::F10), _)        => cd::Special(cd::F10),
        (_, Some(tb::F11), _)        => cd::Special(cd::F11),
        (_, Some(tb::F12), _)        => cd::Special(cd::F12),
        (_, Some(tb::Insert), _)     => cd::Special(cd::Insert),
        (_, Some(tb::Delete), _)     => cd::Special(cd::Delete),
        (_, Some(tb::Home), _)       => cd::Special(cd::Home),
        (_, Some(tb::End), _)        => cd::Special(cd::End),
        (_, Some(tb::Pgup), _)       => cd::Special(cd::PgUp),
        (_, Some(tb::Pgdn), _)     => cd::Special(cd::PgDown),
        (_, Some(tb::ArrowUp), _)    => cd::Special(cd::ArrowUp),
        (_, Some(tb::ArrowDown), _)  => cd::Special(cd::ArrowDown),
        (_, Some(tb::ArrowLeft), _)  => cd::Special(cd::ArrowLeft),
        (_, Some(tb::ArrowRight), _) => cd::Special(cd::ArrowRight),
        (_, Some(tb::Backspace), _)  => cd::Special(cd::Backspace),
        (_, Some(tb::Backspace2), _)  => cd::Special(cd::Backspace),
        (_, Some(tb::Tab), _)        => cd::Special(cd::Tab),
        (_, Some(tb::Enter), _)      => cd::Special(cd::Enter),
        (_, Some(tb::Space), _)      => cd::Character(' '),
        (_, _, Some(c))              => cd::Character(c),
        (_, Some(tb::CtrlTilde), _)  => cd::Special(cd::CtrlTilde),
        // TODO(TyOverby): Handle more cases.  Notably ctrl+`x`
        _ =>  { return None; }
    })
}
