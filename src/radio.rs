use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Radio {
    name: String,
    value: String,
    label: String,
    checked: bool,
}

impl Radio {
    pub fn new(
        name: impl Into<String>,
        value: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            label: label.into(),
            checked: false,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
}

impl Render for Radio {
    fn render(&self) -> String {
        let checked_attr = if self.checked { " checked" } else { "" };
        format!(
            "<label class=\"tui-radio\"><input type=\"radio\" name=\"{}\" value=\"{}\"{checked_attr}><span class=\"tui-radio__circle\"></span>{}</label>",
            self.name, self.value, self.label
        )
    }
}
