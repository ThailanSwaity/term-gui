use crossterm::{cursor, style, QueueableCommand};
use std::io::{Stdout, Write};

use std::error::Error;

pub struct Window {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    title: String,
    text_content: String,
}

impl Window {
    pub fn new(x: u16, y: u16, width: u16, height: u16, title: &str, text_content: &str) -> Self {
        Window {
            x,
            y,
            width,
            height,
            title: String::from(title),
            text_content: String::from(text_content),
        }
    }
}

pub fn draw_window(stdout: &mut Stdout, window: &Window) -> Result<(), Box<dyn Error>> {
    draw_border(stdout, window)?;
    draw_title(stdout, window)?;
    draw_content(stdout, window)?;
    Ok(())
}

fn draw_border(stdout: &mut Stdout, window: &Window) -> Result<(), Box<dyn Error>> {
    for dy in 0..window.height {
        for dx in 0..window.width {
            if dy == 0 && dx == 0 {
                stdout
                    .queue(cursor::MoveTo(window.x + dx, window.y + dy))?
                    .queue(style::Print('╔'))?;
            } else if (dy == 0 || dy == window.height - 1) && (dx != 0 && dx != window.width - 1) {
                stdout
                    .queue(cursor::MoveTo(window.x + dx, window.y + dy))?
                    .queue(style::Print('═'))?;
            } else if dy == 0 && dx == window.width - 1 {
                stdout
                    .queue(cursor::MoveTo(window.x + dx, window.y + dy))?
                    .queue(style::Print('╗'))?;
            } else if (dx == 0 || dx == window.width - 1) && (dy != 0 && dy != window.height - 1) {
                stdout
                    .queue(cursor::MoveTo(window.x + dx, window.y + dy))?
                    .queue(style::Print('║'))?;
            } else if dy == window.height - 1 && dx == 0 {
                stdout
                    .queue(cursor::MoveTo(window.x + dx, window.y + dy))?
                    .queue(style::Print('╚'))?;
            } else if dy == window.height - 1 && dx == window.width - 1 {
                stdout
                    .queue(cursor::MoveTo(window.x + dx, window.y + dy))?
                    .queue(style::Print('╝'))?;
            }
        }
    }
    stdout.flush()?;
    Ok(())
}

fn draw_title(stdout: &mut Stdout, window: &Window) -> Result<(), Box<dyn Error>> {
    stdout
        .queue(cursor::MoveTo(window.x + 2, window.y))?
        .queue(style::Print(format!(" {} ", &window.title)))?;
    Ok(())
}

fn draw_content(stdout: &mut Stdout, window: &Window) -> Result<(), Box<dyn Error>> {
    // TODO: implement alignment options
    draw_text_with_wrap(
        stdout,
        &window.text_content,
        window.x + 2,
        window.y + 1,
        window.width - 4,
    )?;
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
