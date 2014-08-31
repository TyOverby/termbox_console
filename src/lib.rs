extern crate termbox;
extern crate console_draw;

use termbox as tb;
use console_draw as cd;

mod trans;

pub struct TermboxConsole {
    style: tb::Style,
    fg_color: tb::Color,
    bg_color: tb::Color
}

impl TermboxConsole {
    pub fn new() -> TermboxConsole {
        tb::init();
        TermboxConsole {
            style: tb::Normal,
            fg_color: tb::White,
            bg_color: tb::Black
        }
    }
}

impl Drop for TermboxConsole {
    fn drop(&mut self) {
        tb::shutdown();
    }
}

impl cd::ConsoleCanvas for TermboxConsole {
    fn add_modifier(&mut self, modifier: &cd::Modifier) {
        match (self.style, *modifier) {
            (_, cd::TextColor(c)) => {
                self.fg_color = trans::color(c);
            }
            (_, cd::BackgroundColor(c)) => {
                self.bg_color = trans::color(c);
            }
            (tb::Bold, cd::Underline) => {
                self.style = tb::BoldUnderline;
            }
            (tb::Underline, cd::Bold) => {
                self.style = tb::BoldUnderline;
            }
            (_, cd::Bold) => {
                self.style = tb::Bold;
            }
            (_, cd::Underline) => {
                self.style = tb::Underline;
            }

        }
    }

    fn draw_char(&mut self, x: uint, y: uint, c: char) {
        tb::print_ch(x, y, self.style, self.fg_color, self.bg_color, c);
    }

    fn clear(&mut self) {
        tb::clear();
    }

    fn present(&mut self) {
        tb::present();
    }

    fn clear_modifiers(&mut self) {
        self.bg_color = tb::Black;
        self.fg_color = tb::White;
        self.style = tb::Normal;
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
