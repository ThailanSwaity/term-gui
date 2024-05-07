use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::stdout;

use std::error::Error;
use std::process;
use std::{thread, time};

use term_gui::{Alignment, Options, Window};

struct Config {
    cols: u16,
    rows: u16,
}

fn main() {
    let (cols, rows) = terminal::size().unwrap();

    if let Err(e) = run(Config { cols, rows }) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    let mut main_window = Window::new(0, 0, config.cols, config.rows, "", "");

    let mut child_window1 = Window::new(0, 0, 40, 20, "Child Window 1", "");
    main_window.set_child(&child_window1);

    let child_window2 = Window::new(0, 0, 20, 10, "Child Window 2", "");
    child_window1.set_child(&child_window2);

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    // draw windows
    Window::draw_root(&mut stdout, &main_window);

    stdout.execute(cursor::MoveTo(0, config.rows))?;

    Ok(())
}
