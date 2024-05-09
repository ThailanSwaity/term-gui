pub mod draw;

pub enum Alignment {
    Min,
    Center,
    Max,
    None,
}

pub struct Options {
    pub vertical_align: Alignment,
    pub horizontal_align: Alignment,
    pub vertical_text_align: Alignment,
    pub render_border: bool,
    pub render_content: bool,
}

pub struct Window {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    title: String,
    text_content: String,
    children: Vec<Window>,
    pub options: Options,
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
                vertical_text_align: Alignment::Min,
                render_border: true,
                render_content: true,
            },
        }
    }

    pub fn fit_text(&mut self) {
        let text_height = compute_text_height(&self.text_content, self.width - 2);
        self.height = text_height + 3;
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

pub fn compute_text_height(text: &str, wrapping_width: u16) -> u16 {
    if text.len() == 0 {
        return 0;
    }
    let mut text_width = 0;
    let mut text_height = 1;
    for word in text.split_whitespace() {
        if text_width + word.len() as u16 > wrapping_width {
            text_height += 1;
            text_width = 0;
        }
        text_width += word.len() as u16 + 1;
    }
    text_height
}
