# Termbox Console
#### An implementation of the [`console_draw`](https://github.com/TyOverby/console_draw) API using the termbox backend

If you are thinking of using ncurses, but all you really need is the ability to

1. draw arbitrary chars or strings to the console
2. get keyboard input events
3. draw a cursor on the screen

then Termbox Console will fit your needs perfectly!

[Api Documentation](http://tyoverby.com/termbox_console/termbox_console/index.html)

## Examples
These examples can also be found in the `examples` directory.
### Hello World
Draw the text '"hello world"' to the screen at a specific location.
```rust
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

```

### Marquee
Bounce some text back and forth until the user presses 'q'.
```rust
extern crate termbox_console;
extern crate console_draw;

use std::io::timer::sleep;
use std::time::duration::Duration;

use console_draw::{
    ConsoleCanvas,
    Character,
    Resize
};
use termbox_console::TermboxConsole;

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

```

### Text Input
Draw what the user types.
```rust
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

```
