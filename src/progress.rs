use crate::render::Render;

#[derive(Clone, Debug)]
pub struct ProgressBar {
    value: u8,
}

impl ProgressBar {
    pub fn new(value: u8) -> Self {
        Self {
            value: value.clamp(0, 100),
        }
    }
}

impl Render for ProgressBar {
    fn render(&self) -> String {
        format!(
            "<div class=\"tui-progress\" role=\"progressbar\" aria-valuenow=\"{}\" aria-valuemin=\"0\" aria-valuemax=\"100\"><div class=\"tui-progress__bar\" style=\"width: {}%;\"></div></div>",
            self.value, self.value
        )
    }
}

#[derive(Clone, Debug)]
pub struct ProgressRing;

impl ProgressRing {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProgressRing {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ProgressRing {
    fn render(&self) -> String {
        String::from(
            "<div class=\"tui-progress-ring\" role=\"status\" aria-label=\"loading\"></div>",
        )
    }
}
