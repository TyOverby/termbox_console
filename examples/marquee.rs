extern crate console_draw_termbox;
extern crate console_draw;

use std::io::timer::sleep;
use std::time::duration::Duration;

use console_draw::{
    ConsoleCanvas,
    Character,
    Resize
};
use console_draw_termbox::TermboxConsole;

fn main() {
    let mut console = TermboxConsole::new();

    let message = "<marquee> Hello World </marquee>";
    let mut width = console.width();
    let mut pos = 0i;
    let mut direction = 1i;

    'outer: loop {
        console.clear();
        console.draw(pos as uint, 0, message);
        console.draw(5, 5, "press 'q' to quit");
        console.present();

        pos += direction;
        if pos == 0 || pos as uint + message.len() > width - 1 {
            direction *= -1;
        }

        for update in console {
            match update {
                Character('q') => { break 'outer; }
                Resize(w, _) => { width = w; }
                _ => { }
            }
        }

        sleep(Duration::milliseconds(50));
    }
}
