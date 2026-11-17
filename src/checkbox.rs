use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Checkbox {
    label: String,
    checked: bool,
    disabled: bool,
}

impl Checkbox {
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

impl Render for Checkbox {
    fn render(&self) -> String {
        let checked_attr = if self.checked { " checked" } else { "" };
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        format!(
            "<label class=\"tui-checkbox\"><input type=\"checkbox\"{checked_attr}{disabled_attr}><span class=\"tui-checkbox__box\"></span>{}</label>",
            self.label
        )
    }
}
