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

    let main_window = Window::new(0, 0, config.cols, config.rows, "", "");

    let mut moving_window1 = Window::new(0, 0, 80, 40, "Moving Window 1", "");
    moving_window1.set_parent(&main_window);
    moving_window1.set_options(Options {
        vertical_align: Alignment::Center,
        horizontal_align: Alignment::Center,
    });

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    // draw windows
    term_gui::draw_window(&mut stdout, &moving_window1)?;

    stdout.execute(cursor::MoveTo(0, config.rows))?;

    Ok(())
}
