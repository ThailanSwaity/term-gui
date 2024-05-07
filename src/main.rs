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

    let child_window = Window::new(0, 0, 30, 15, "Child Window", "child window text");

    let parent_window = Window::new(10, 5, 50, 30, "Parent Window", "");
    let child_window_2 = Window::new(0, 0, 30, 15, "Child Window", "nice.");

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    main_window.draw_as_child(
        &mut stdout,
        &child_window,
        Options {
            vertical_align: Alignment::Center,
            horizontal_align: Alignment::Center,
        },
    )?;

    term_gui::draw_window(&mut stdout, &parent_window)?;
    parent_window.draw_as_child(
        &mut stdout,
        &child_window_2,
        Options {
            vertical_align: Alignment::Center,
            horizontal_align: Alignment::Center,
        },
    )?;

    stdout.execute(cursor::MoveTo(0, config.rows))?;

    Ok(())
}
