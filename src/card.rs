use crate::render::Render;

#[derive(Clone, Debug, Default)]
pub struct Card {
    title: Option<String>,
    body: String,
}

impl Card {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }
}

impl Render for Card {
    fn render(&self) -> String {
        let title = self
            .title
            .as_ref()
            .map(|t| format!("<h3 class=\"tui-card__title\">{t}</h3>"))
            .unwrap_or_default();
        format!(
            "<div class=\"tui-card\">{title}<div class=\"tui-card__body\">{}</div></div>",
            self.body
        )
    }
}
