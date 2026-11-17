use crate::render::Render;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TagColor {
    #[default]
    Neutral,
    Primary,
    Success,
    Warning,
    Danger,
}

impl TagColor {
    pub fn class_suffix(self) -> &'static str {
        match self {
            TagColor::Neutral => "",
            TagColor::Primary => "tui-tag--primary",
            TagColor::Success => "tui-tag--success",
            TagColor::Warning => "tui-tag--warning",
            TagColor::Danger => "tui-tag--danger",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tag {
    text: String,
    color: TagColor,
}

impl Tag {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: TagColor::Neutral,
        }
    }

    pub fn color(mut self, color: TagColor) -> Self {
        self.color = color;
        self
    }
}

impl Render for Tag {
    fn render(&self) -> String {
        let mut classes = String::from("tui-tag");
        let suffix = self.color.class_suffix();
        if !suffix.is_empty() {
            classes.push(' ');
            classes.push_str(suffix);
        }
        format!("<span class=\"{classes}\">{}</span>", self.text)
    }
}
