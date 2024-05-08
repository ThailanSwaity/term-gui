use crate::{Alignment, Window};

use crossterm::{cursor, style, QueueableCommand};
use std::io::Stdout;

use std::error::Error;

pub fn draw_window_tree(
    stdout: &mut Stdout,
    window: &Window,
    draw_root: bool,
) -> Result<(), Box<dyn Error>> {
    draw(stdout, 0, 0, window.width, window.height, window, draw_root)?;
    Ok(())
}

fn draw(
    stdout: &mut Stdout,
    origin_x: u16,
    origin_y: u16,
    parent_width: u16,
    parent_height: u16,
    window: &Window,
    draw_window: bool,
) -> Result<(), Box<dyn Error>> {
    let mut absolute_x = origin_x + window.x;
    let mut absolute_y = origin_y + window.y;

    match window.options.vertical_align {
        Alignment::Min => {
            absolute_y = origin_y;
        }
        Alignment::Center => {
            absolute_y = origin_y + (parent_height / 2) - (window.height / 2) - 1;
        }
        Alignment::Max => {
            absolute_y = origin_y + parent_height - window.height - 2;
        }
        Alignment::None => {}
    }

    match window.options.horizontal_align {
        Alignment::Min => {
            absolute_x = origin_x;
        }
        Alignment::Center => {
            absolute_x = origin_x + (parent_width / 2) - (window.width / 2) - 1;
        }
        Alignment::Max => {
            absolute_x = origin_x + parent_width - window.width - 2;
        }
        Alignment::None => {}
    }

    if draw_window {
        draw_border(stdout, absolute_x, absolute_y, window.width, window.height)?;
        if window.title != "" {
            draw_title(
                stdout,
                &window.title,
                absolute_x,
                absolute_y,
                window.width,
                window.height,
            )?;
        }
        draw_content(
            stdout,
            &window.text_content,
            absolute_x,
            absolute_y,
            window.width,
            window.height,
        )?;
    }

    for child in window.get_children() {
        draw(
            stdout,
            absolute_x + 1,
            absolute_y + 1,
            window.width,
            window.height,
            child,
            true,
        )?;
    }
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
