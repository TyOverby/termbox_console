extern crate termbox;
extern crate console_draw;

use termbox as tb;
use console_draw as cd;

mod trans;

#[deriving(Copy)]
struct Style {
    font_options: tb::Style,
    fg_color: tb::Color,
    bg_color: tb::Color
}
pub struct TermboxConsole {
    current: Style,
    stack: Vec<Style>
}

impl TermboxConsole {
    pub fn new() -> TermboxConsole {
        tb::init();
        TermboxConsole {
            current: Style {
                font_options: tb::Normal,
                fg_color: tb::White,
                bg_color: tb::Black
            },
            stack: vec![]
        }
    }
}

impl Drop for TermboxConsole {
    fn drop(&mut self) {
        tb::shutdown();
    }
}

impl cd::ConsoleCanvas for TermboxConsole {
    fn set(&mut self, modifier: cd::Modifier) {
        match (self.current.font_options, modifier) {
            (_, cd::TextColor(c)) => {
                self.current.fg_color = trans::color(c);
            }
            (_, cd::BackgroundColor(c)) => {
                self.current.bg_color = trans::color(c);
            }
            (tb::Bold, cd::Underline) => {
                self.current.font_options = tb::BoldUnderline;
            }
            (tb::Underline, cd::Bold) => {
                self.current.font_options = tb::BoldUnderline;
            }
            (_, cd::Bold) => {
                self.current.font_options = tb::Bold;
            }
            (_, cd::Underline) => {
                self.current.font_options = tb::Underline;
            }
        }
    }

    fn draw_char(&mut self, x: uint, y: uint, c: char) {
        tb::print_ch(x, y,
                     self.current.font_options,
                     self.current.fg_color,
                     self.current.bg_color, c);
    }

    fn clear(&mut self) {
        tb::clear();
    }

    fn present(&mut self) {
        tb::present();
    }

    fn push_state(&mut self) {
        let cloned = self.current;
        self.stack.push(cloned);
    }
    fn pop_state(&mut self) {
        match self.stack.pop() {
            Some(s) => self.current = s,
            None => {}
        }
    }

    fn supports_custom_colors(&self) -> bool { false }

    fn width(&self) -> uint {
        tb::width()
    }

    fn height(&self) -> uint {
        tb::height()
    }

    fn cursor(&mut self, x: uint, y: uint) {
        tb::set_cursor(x, y);
    }
}

impl Iterator<cd::Update> for TermboxConsole {
    fn next(&mut self) -> Option<cd::Update> {
        match tb::peek_event(0) {
            tb::KeyEvent(i, key, chr) => trans::key_event(i, key, chr),
            tb::ResizeEvent(w, h) => Some(cd::Resize(w as uint, h as uint)),
            tb::NoEvent => None
        }
    }
}

impl cd::ConsoleInput for TermboxConsole { }
