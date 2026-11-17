use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Toggle {
    label: String,
    checked: bool,
    disabled: bool,
}

impl Toggle {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            checked: false,
            disabled: false,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Render for Toggle {
    fn render(&self) -> String {
        let checked_attr = if self.checked { " checked" } else { "" };
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        format!(
            "<label class=\"tui-toggle\"><input type=\"checkbox\"{checked_attr}{disabled_attr}><span class=\"tui-toggle__track\"><span class=\"tui-toggle__thumb\"></span></span>{}</label>",
            self.label
        )
    }
}
