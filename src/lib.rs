use crossterm::{cursor, style, QueueableCommand};
use std::io::{Stdout, Write};

use std::error::Error;

pub enum Alignment {
    Min,
    Center,
    Max,
    None,
}

pub struct Options {
    pub vertical_align: Alignment,
    pub horizontal_align: Alignment,
}

// TODO: give this an optional reference to a parent window.
// Then use this parent reference to compute where to draw itself.
pub struct Window<'a> {
    pub x: u16,
    pub y: u16,
    width: u16,
    height: u16,
    title: String,
    text_content: String,
    parent: Option<&'a Window<'a>>,
    options: Options,
}

impl<'a> Window<'a> {
    pub fn new(x: u16, y: u16, width: u16, height: u16, title: &str, text_content: &str) -> Self {
        Window {
            x,
            y,
            width,
            height,
            title: String::from(title),
            text_content: String::from(text_content),
            parent: None,
            options: Options {
                vertical_align: Alignment::None,
                horizontal_align: Alignment::None,
            },
        }
    }

    pub fn set_parent(&mut self, parent: &'a Window) {
        self.parent = Some(parent);
    }

    pub fn set_options(&mut self, options: Options) {
        self.options = options;
    }
}

pub fn draw_window(stdout: &mut Stdout, window: &Window) -> Result<(), Box<dyn Error>> {
    if let Some(parent_window) = window.parent {
        let origin_x = parent_window.x;
        let origin_y = parent_window.y;

        let mut relative_x;
        let mut relative_y;
        match window.options.vertical_align {
            Alignment::Min => {
                relative_y = origin_y + 1;
            }
            Alignment::Center => {
                relative_y = origin_y + (parent_window.height / 2) - (window.height / 2);
            }
            Alignment::Max => {
                relative_y = origin_y + parent_window.height - window.height - 1;
            }
            Alignment::None => {
                relative_y = origin_y + window.y + 1;
                // TODO: restict child window to inside of parent window
            } // TODO: add margins
        }

        match window.options.horizontal_align {
            Alignment::Min => {
                relative_x = origin_x + 1;
            }
            Alignment::Center => {
                relative_x = origin_x + (parent_window.width / 2) - (window.width / 2);
            }
            Alignment::Max => {
                relative_x = origin_x + parent_window.width - window.width - 1;
            }
            Alignment::None => {
                relative_x = origin_x + window.x + 1;
                // TODO: restict child window to inside of parent window
            } // TODO: add margins
        }

        draw_border(stdout, relative_x, relative_y, window.width, window.height)?;
        draw_title(
            stdout,
            &window.title,
            relative_x,
            relative_y,
            window.width,
            window.height,
        )?;
        draw_content(
            stdout,
            &window.text_content,
            relative_x,
            relative_y,
            window.width,
            window.height,
        )?;
    }
    if window.parent.is_none() {
        draw_border(stdout, window.x, window.y, window.width, window.height)?;
        draw_title(
            stdout,
            &window.title,
            window.x,
            window.y,
            window.width,
            window.height,
        )?;
        draw_content(
            stdout,
            &window.text_content,
            window.x,
            window.y,
            window.width,
            window.height,
        )?;
    }

    stdout.flush()?;
    Ok(())
}

fn draw_border(
    stdout: &mut Stdout,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) -> Result<(), Box<dyn Error>> {
    for dy in 0..height {
        for dx in 0..width {
            if dy == 0 && dx == 0 {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))?
                    .queue(style::Print('╔'))?;
            } else if (dy == 0 || dy == height - 1) && (dx != 0 && dx != width - 1) {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))?
                    .queue(style::Print('═'))?;
            } else if dy == 0 && dx == width - 1 {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))?
                    .queue(style::Print('╗'))?;
            } else if (dx == 0 || dx == width - 1) && (dy != 0 && dy != height - 1) {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))?
                    .queue(style::Print('║'))?;
            } else if dy == height - 1 && dx == 0 {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))?
                    .queue(style::Print('╚'))?;
            } else if dy == height - 1 && dx == width - 1 {
                stdout
                    .queue(cursor::MoveTo(x + dx, y + dy))?
                    .queue(style::Print('╝'))?;
            }
        }
    }
    Ok(())
}

fn draw_title(
    stdout: &mut Stdout,
    title: &str,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) -> Result<(), Box<dyn Error>> {
    stdout
        .queue(cursor::MoveTo(x + 2, y))?
        .queue(style::Print(format!(" {} ", title)))?;
    Ok(())
}

fn draw_content(
    stdout: &mut Stdout,
    text_content: &str,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) -> Result<(), Box<dyn Error>> {
    // TODO: implement alignment options
    draw_text_with_wrap(stdout, text_content, x + 2, y + 1, width - 4)?;
    Ok(())
}

fn draw_text_with_wrap(
    stdout: &mut Stdout,
    text: &str,
    x: u16,
    y: u16,
    width: u16,
) -> Result<(), Box<dyn Error>> {
    let mut dy = 0;
    let mut dx = 0;
    for word in text.split_whitespace() {
        if dx + word.len() as u16 > width {
            dy += 1;
            dx = 0;
        }
        stdout
            .queue(cursor::MoveTo(x + dx, y + dy))?
            .queue(style::Print(&word))?;
        dx += word.len() as u16 + 1;
    }
    Ok(())
}
