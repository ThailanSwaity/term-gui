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

pub struct Window {
    pub x: u16,
    pub y: u16,
    width: u16,
    height: u16,
    title: String,
    text_content: String,
    children: Vec<Window>,
    options: Options,
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
            children: Vec::new(),
            options: Options {
                vertical_align: Alignment::None,
                horizontal_align: Alignment::None,
            },
        }
    }

    pub fn set_options(&mut self, options: Options) {
        self.options = options;
    }

    pub fn add_child(&mut self, window: Window) {
        self.children.push(window);
    }

    pub fn get_children(&self) -> &Vec<Window> {
        &self.children
    }

    pub fn get_children_as_mut(&mut self) -> &mut Vec<Window> {
        &mut self.children
    }
}

pub fn draw_window_tree(stdout: &mut Stdout, window: &Window) -> Result<(), Box<dyn Error>> {
    draw(stdout, 0, 0, window.width, window.height, window)?;
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
