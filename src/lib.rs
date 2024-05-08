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
}

pub struct Window {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
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
                vertical_text_align: Alignment::Min,
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
