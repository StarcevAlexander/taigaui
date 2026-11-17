use crate::render::Render;

#[derive(Clone, Debug)]
pub struct Tab {
    label: String,
    active: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Tabs {
    tabs: Vec<Tab>,
}

impl Tabs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tab(mut self, label: impl Into<String>, active: bool) -> Self {
        self.tabs.push(Tab {
            label: label.into(),
            active,
        });
        self
    }
}

impl Render for Tabs {
    fn render(&self) -> String {
        let mut out = String::from("<div class=\"tui-tabs\" role=\"tablist\">");
        for tab in &self.tabs {
            let mut classes = String::from("tui-tabs__item");
            if tab.active {
                classes.push_str(" tui-tabs__item--active");
            }
            out.push_str(&format!(
                "<button class=\"{classes}\" type=\"button\" role=\"tab\" aria-selected=\"{}\">{}</button>",
                tab.active, tab.label
            ));
        }
        out.push_str("</div>");
        out
    }
}
