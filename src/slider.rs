use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Slider {
    name: String,
    min: f64,
    max: f64,
    value: f64,
}

impl Slider {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            min: 0.0,
            max: 100.0,
            value: 50.0,
        }
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }
}

impl Render for Slider {
    fn render(&self) -> String {
        let span = if (self.max - self.min).abs() < f64::EPSILON {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min) * 100.0
        };
        format!(
            "<input type=\"range\" class=\"tui-slider\" name=\"{}\" min=\"{}\" max=\"{}\" value=\"{}\" style=\"--tui-slider-value: {span:.0}%;\">",
            self.name, self.min, self.max, self.value
        )
    }
}
