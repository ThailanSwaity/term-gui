use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::stdout;

use std::error::Error;
use std::process;

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

    let mut root_window = Window::new(0, 0, config.cols, config.rows - 20, "root", "root content");
    let mut t_window = Window::new(0, 0, 40, 20, "child 1", "child 1 content");
    t_window.set_options(Options {
        vertical_align: Alignment::Center,
        horizontal_align: Alignment::Center,
    });
    root_window.add_child(t_window);

    // draw windows
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let mut t_window = Window::new(0, 0, 20, 10, "child 2", "child 2 content");
    t_window.set_options(Options {
        vertical_align: Alignment::Max,
        horizontal_align: Alignment::Max,
    });

    root_window.add_child(t_window);

    term_gui::draw_window_tree(&mut stdout, &root_window)?;

    stdout.execute(cursor::MoveTo(0, config.rows))?;

    Ok(())
}
