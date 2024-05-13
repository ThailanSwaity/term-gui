use crate::{compute_text_height, Alignment, Options, Window};

use crossterm::{cursor, style, QueueableCommand};
use std::io::{stdout, Stdout, Write};

use std::error::Error;

pub fn draw_window_tree(window: &Window) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    draw(&mut stdout, 0, 0, window.width, window.height, window)?;
    Ok(())
}

fn draw(
    stdout: &mut Stdout,
    origin_x: u16,
    origin_y: u16,
    parent_width: u16,
    parent_height: u16,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    let mut absolute_x = origin_x + window.x;
    let mut absolute_y = origin_y + window.y;

    match window.options.vertical_align {
        Alignment::Min(offset) => {
            if offset >= 0 {
                absolute_y = origin_y + offset as u16;
            }
        }
        Alignment::Center(offset) => {
            let spot_y =
                origin_y as i16 + (parent_height / 2) as i16 - (window.height / 2) as i16 - 1
                    + offset;
            if spot_y >= 0 {
                absolute_y = spot_y as u16;
            }
        }
        Alignment::Max(offset) => {
            let spot_y = origin_y as i16 + parent_height as i16 - window.height as i16 - 2 + offset;
            if offset <= 0 && spot_y >= 0 {
                absolute_y = spot_y as u16;
            }
        }
        Alignment::None => {}
    }

    match window.options.horizontal_align {
        Alignment::Min(offset) => {
            if offset >= 0 {
                absolute_x = origin_x + offset as u16;
            }
        }
        Alignment::Center(offset) => {
            let spot_x =
                origin_x as i16 + (parent_width / 2) as i16 - (window.width / 2) as i16 - 1
                    + offset;
            if spot_x >= 0 {
                absolute_x = spot_x as u16;
            }
        }
        Alignment::Max(offset) => {
            let spot_x = origin_x as i16 + parent_width as i16 - window.width as i16 - 2 + offset;
            if offset <= 0 && spot_x >= 0 {
                absolute_x = spot_x as u16;
            }
        }
        Alignment::None => {}
    }

    if window.options.render_border {
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
    }

    if window.options.render_content {
        draw_content(
            stdout,
            &window.text_content,
            absolute_x + 1,
            absolute_y + 1,
            window.width - 2,
            window.height - 2,
            &window.options,
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
    options: &Options,
) -> Result<(), Box<dyn Error>> {
    let text_width = width - 2 * options.horizontal_text_padding;
    let text_height = compute_text_height(text_content, text_width);

    let mut absolute_y = y;
    match options.vertical_text_align {
        Alignment::Center(offset) => {
            let spot_y = y as i16 + (height / 2) as i16 - (text_height / 2) as i16 + offset;
            if spot_y >= 0 {
                absolute_y = spot_y as u16;
            }
        }
        Alignment::Max(offset) => {
            let spot_y = y as i16 + height as i16 - text_height as i16 + offset;
            if offset <= 0 && spot_y >= 0 {
                absolute_y = spot_y as u16;
            }
        }
        Alignment::Min(offset) => {
            if offset >= 0 {
                absolute_y = y + offset as u16;
            }
        }
        Alignment::None => {}
    }

    draw_text_with_wrap(
        stdout,
        text_content,
        x + options.horizontal_text_padding,
        absolute_y,
        text_width,
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
    //    stdout.queue(style::Print(compute_text_height(text, width)))?;
    Ok(())
}
