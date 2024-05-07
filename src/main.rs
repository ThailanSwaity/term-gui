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

    let mut root_window = Window::new(0, 0, config.cols, config.rows, "root", "root content");
    root_window.add_child(Window::new(0, 0, 40, 20, "child 1", "child 1 content"));

    // draw windows
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let children_ref = &mut root_window.get_children_as_mut()[0];
    children_ref.x = 10;

    term_gui::draw_window_tree(&mut stdout, &root_window)?;

    stdout.execute(cursor::MoveTo(0, config.rows))?;

    Ok(())
}
