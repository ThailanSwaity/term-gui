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

pub struct Window<'a> {
    pub x: u16,
    pub y: u16,
    width: u16,
    height: u16,
    title: String,
    text_content: String,
    child: Option<&'a mut Window<'a>>,
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
            child: None,
            options: Options {
                vertical_align: Alignment::None,
                horizontal_align: Alignment::None,
            },
        }
    }

    pub fn set_child(&mut self, child: &'a mut Window<'a>) {
        self.child = Some(child);
    }

    pub fn set_options(&mut self, options: Options) {
        self.options = options;
    }

    pub fn draw_root(stdout: &mut Stdout, window: &Window) -> Result<(), Box<dyn Error>> {
        window.draw(stdout, 0, 0)?;
        Ok(())
    }

    fn draw(
        &self,
        stdout: &mut Stdout,
        origin_x: u16,
        origin_y: u16,
    ) -> Result<(), Box<dyn Error>> {
        let absolute_x = origin_x + self.x;
        let absolute_y = origin_y + self.y;
        draw_border(stdout, absolute_x, absolute_y, self.width, self.height)?;
        draw_title(
            stdout,
            &self.title,
            absolute_x,
            absolute_y,
            self.width,
            self.height,
        )?;
        draw_content(
            stdout,
            &self.text_content,
            absolute_x,
            absolute_y,
            self.width,
            self.height,
        )?;
        if let Some(child_window) = self.child {
            child_window.draw(stdout, absolute_x, absolute_y)?;
        }
        Ok(())
    }
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
