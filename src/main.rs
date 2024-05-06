use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::stdout;

use std::process;

use term_gui::Window;

fn main() {
    let (cols, rows) = terminal::size().unwrap();
    let window = Window::new(1, 10, cols - 8, 15, "A Short Story", "Short story about a friend that got into too much trouble; he couldn't help it, so he said. However, the rest of us knew he just didn't want to. He was not willing to put in the effort to make change. He kept up his ways, slowly alienating himself from the group. Not one of his friends could stand his behaviour any longer! They couldn't stand his excessive talking.");
    let window2 = Window::new(50, 3, 35, 5, "Nothing to see here.", "Nothing to see here.");

    let mut stdout = stdout();

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();

    if let Err(e) = term_gui::draw_window(&mut stdout, &window) {
        eprintln!("Applicaiton error: {e}");
        process::exit(1);
    }

    if let Err(e) = term_gui::draw_window(&mut stdout, &window2) {
        eprintln!("Applicaiton error: {e}");
        process::exit(1);
    }

    stdout.execute(cursor::MoveTo(0, rows)).unwrap();
}
