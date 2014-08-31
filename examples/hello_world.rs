extern crate console_draw_termbox;
extern crate console_draw;

use std::io::timer::sleep;
use std::time::duration::Duration;

use console_draw::ConsoleCanvas;
use console_draw_termbox::TermboxConsole;

fn main() {
    let mut console = TermboxConsole::new();
    console.draw(5, 5, "hello world");
    console.present();
    sleep(Duration::milliseconds(1000));
}
