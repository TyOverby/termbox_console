# Termbox Console
### An implementation of the [`console_draw`](https://github.com/TyOverby/console_draw)
API using the termbox backend

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
^code(./examples/hello_world.rs)

### Marquee
Bounce some text back and forth until the user presses 'q'.
^code(./examples/marquee.rs)

### Text Input
Draw what the user types.
^code(./examples/text_input.rs)
