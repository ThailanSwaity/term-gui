use crossterm::{terminal, ExecutableCommand};
use std::io::stdout;
use std::{thread, time};

use std::error::Error;
use std::process;

use term_gui::{draw::draw_window_tree, Alignment, Options, Window};

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
    root_window.options = Options {
        vertical_align: Alignment::None,
        horizontal_align: Alignment::None,
        vertical_text_align: Alignment::Center,
        render_border: true,
        render_content: true,
    };
    let mut t_window = Window::new(
        0,
        0,
        40,
        20,
        "child 1",
        "child 1 content, with wrap and alignment :)",
    );
    t_window.options = Options {
        vertical_align: Alignment::Center,
        horizontal_align: Alignment::Center,
        vertical_text_align: Alignment::Max,
        render_border: false,
        render_content: true,
    };
    let mut t2_window = Window::new(5, 5, 20, 9, "child 3", "child 3 content");
    t2_window.options = Options {
        vertical_align: Alignment::None,
        horizontal_align: Alignment::None,
        vertical_text_align: Alignment::Center,
        render_border: true,
        render_content: true,
    };
    t_window.add_child(t2_window);
    root_window.add_child(t_window);

    let mut t_window = Window::new(0, 0, 20, 9, "child 2", "child 2 content");
    t_window.options = Options {
        vertical_align: Alignment::Max,
        horizontal_align: Alignment::Max,
        vertical_text_align: Alignment::Center,
        render_border: true,
        render_content: true,
    };
    root_window.add_child(t_window);

    let mut t_window = Window::new(0, 15, 35, 5, "child 4", "moving text without a border");
    t_window.options = Options {
        vertical_align: Alignment::None,
        horizontal_align: Alignment::Max,
        vertical_text_align: Alignment::Center,
        render_border: false,
        render_content: true,
    };

    root_window.add_child(t_window);

    let child_1_ref = &mut root_window.get_children_as_mut()[0];
    child_1_ref.options = Options {
        vertical_align: Alignment::None,
        horizontal_align: Alignment::None,
        vertical_text_align: Alignment::Max,
        render_border: true,
        render_content: true,
    };

    let center_x = config.cols / 2;
    let center_y = config.rows / 2;

    let mut theta: f32 = 0.0;
    let mut theta2: f32 = 0.0;
    let mut theta3: f32 = 0.0;

    loop {
        theta += 0.1;
        let child_1_ref = &mut root_window.get_children_as_mut()[0];
        let x = theta.cos() * 5.0 + center_x as f32 - (center_x - child_1_ref.width / 2) as f32;
        let y = theta.sin() * 2.5 + center_y as f32 - (center_y - child_1_ref.height / 2) as f32;
        child_1_ref.x = x as u16;
        child_1_ref.y = y as u16;

        theta2 += 0.3;
        let child_3_ref = &mut child_1_ref.get_children_as_mut()[0];
        let x2 = theta2.cos() * 5.0 + center_x as f32 - (center_x - child_3_ref.width / 2) as f32;
        let y2 = theta2.sin() * 2.5 + center_y as f32 - (center_y - child_3_ref.height / 2) as f32;
        child_3_ref.x = x2 as u16;
        child_3_ref.y = y2 as u16;

        theta3 += 0.2;
        let child_4_ref = &mut root_window.get_children_as_mut()[2];
        let anchor_y = theta3.sin() * 10.0 + 15.0;
        child_4_ref.y = anchor_y as u16;

        // draw windows
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        draw_window_tree(&root_window)?;

        thread::sleep(time::Duration::from_millis(100));
    }
}
