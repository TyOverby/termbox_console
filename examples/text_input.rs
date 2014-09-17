extern crate termbox_console;
extern crate console_draw;

use console_draw::{
    ConsoleCanvas,
    Character,
    Special,
    Enter,
    Backspace
};
use termbox_console::TermboxConsole;

fn main() {
    let mut console = TermboxConsole::new();
    let mut buffer = String::new();
    'outer: loop {

        console.clear();
        console.draw(0, 0, buffer.as_slice());
        console.draw(5, 5, "press [enter] to quit");
        console.present();

        for update in console {
            match update {
                Special(Enter) => {
                    break 'outer;
                }
                Special(Backspace) => {
                    buffer.pop_char();
                }
                Character(x) =>  {
                    buffer.push_char(x);
                }
                _ => { }
            }
        }
    }
}
