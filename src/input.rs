use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Input {
    label: String,
    placeholder: String,
    name: String,
    value: String,
    disabled: bool,
}

impl Input {
    pub fn new(label: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            placeholder: placeholder.into(),
            name: String::new(),
            value: String::new(),
            disabled: false,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Render for Input {
    fn render(&self) -> String {
        let name_attr = if self.name.is_empty() {
            String::new()
        } else {
            format!(" name=\"{}\"", self.name)
        };
        let value_attr = if self.value.is_empty() {
            String::new()
        } else {
            format!(" value=\"{}\"", self.value)
        };
        let disabled_attr = if self.disabled { " disabled" } else { "" };
        format!(
            "<div class=\"tui-input\"><label class=\"tui-input__label\">{}</label><input class=\"tui-input__field\" type=\"text\"{name_attr}{value_attr}{disabled_attr} placeholder=\"{}\"></div>",
            self.label, self.placeholder
        )
    }
}

#[derive(Clone, Debug)]
pub struct Textarea {
    label: String,
    placeholder: String,
    rows: u8,
}

impl Textarea {
    pub fn new(label: impl Into<String>, placeholder: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            placeholder: placeholder.into(),
            rows: 4,
        }
    }

    pub fn rows(mut self, rows: u8) -> Self {
        self.rows = rows.clamp(2, 20);
        self
    }
}

impl Render for Textarea {
    fn render(&self) -> String {
        format!(
            "<div class=\"tui-input\"><label class=\"tui-input__label\">{}</label><textarea class=\"tui-input__field tui-textarea\" rows=\"{}\" placeholder=\"{}\"></textarea></div>",
            self.label, self.rows, self.placeholder
        )
    }
}
