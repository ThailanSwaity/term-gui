pub mod draw;

// TODO: Implement Alignment offsets, like Center(i16) for easier window positioning
pub enum Alignment {
    Min(i16),
    Center(i16),
    Max(i16),
    None,
}

pub struct Options {
    pub vertical_align: Alignment,
    pub horizontal_align: Alignment,
    pub vertical_text_align: Alignment,
    pub horizontal_text_padding: u16,
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
                vertical_text_align: Alignment::Min(0),
                horizontal_text_padding: 1,
                render_border: true,
                render_content: true,
            },
        }
    }

    pub fn set_text_content(&mut self, text: &str) {
        self.text_content = String::from(text);
    }

    /// Causes the window to shrink to the size of the text (plus whatever horizontal padding)
    /// In this case, the window width acts as a maximum width. The window width will shrink
    /// to the text only if the text fits on one line within the window, otherwise the height will adjust.
    pub fn fit_text(&mut self) {
        // The padding + 1 accounts for the border character taking away from the actual text width
        let max_text_width = self.width - 2 * (self.options.horizontal_text_padding + 1);
        let text_height = compute_text_height(&self.text_content, max_text_width);
        self.height = text_height + 2;

        if text_height == 1 {
            self.width = self.text_content.len() as u16 + 4;
        }
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
    if text.is_empty() {
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
