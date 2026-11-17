use crate::render::Render;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AlertKind {
    #[default]
    Neutral,
    Success,
    Warning,
    Danger,
}

impl AlertKind {
    pub fn class_suffix(self) -> &'static str {
        match self {
            AlertKind::Neutral => "",
            AlertKind::Success => "tui-alert--success",
            AlertKind::Warning => "tui-alert--warning",
            AlertKind::Danger => "tui-alert--danger",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Alert {
    text: String,
    kind: AlertKind,
}

impl Alert {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: AlertKind::Neutral,
        }
    }

    pub fn kind(mut self, kind: AlertKind) -> Self {
        self.kind = kind;
        self
    }
}

impl Render for Alert {
    fn render(&self) -> String {
        let mut classes = String::from("tui-alert");
        let suffix = self.kind.class_suffix();
        if !suffix.is_empty() {
            classes.push(' ');
            classes.push_str(suffix);
        }
        format!(
            "<div class=\"{classes}\" role=\"alert\">{}</div>",
            self.text
        )
    }
}
