extern crate termbox_console;
extern crate console_draw;

use std::io::timer::sleep;
use std::time::duration::Duration;

use console_draw::ConsoleCanvas;
use termbox_console::TermboxConsole;

fn main() {
    let mut console = TermboxConsole::new();
    console.draw(5, 5, "hello world");
    console.present();
    sleep(Duration::milliseconds(1000));
}
