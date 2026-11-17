use crate::render::Render;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonKind {
    #[default]
    Primary,
    Secondary,
    Flat,
    Danger,
}

impl ButtonKind {
    pub fn class_suffix(self) -> &'static str {
        match self {
            ButtonKind::Primary => "",
            ButtonKind::Secondary => "tui-btn--secondary",
            ButtonKind::Flat => "tui-btn--flat",
            ButtonKind::Danger => "tui-btn--danger",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Button {
    text: String,
    kind: ButtonKind,
    round: bool,
    disabled: bool,
    href: Option<String>,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: ButtonKind::Primary,
            round: false,
            disabled: false,
            href: None,
        }
    }

    pub fn kind(mut self, kind: ButtonKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn round(mut self, round: bool) -> Self {
        self.round = round;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn link(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
}

impl Render for Button {
    fn render(&self) -> String {
        let mut classes = String::from("tui-btn");
        let kind_class = self.kind.class_suffix();
        if !kind_class.is_empty() {
            classes.push(' ');
            classes.push_str(kind_class);
        }
        if self.round {
            classes.push_str(" tui-btn--round");
        }
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        match &self.href {
            Some(href) => format!(
                "<a class=\"{classes}\" href=\"{href}\"{disabled_attr}>{}</a>",
                self.text
            ),
            None => format!(
                "<button class=\"{classes}\" type=\"button\"{disabled_attr}>{}</button>",
                self.text
            ),
        }
    }
}
