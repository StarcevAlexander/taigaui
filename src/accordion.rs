use crate::render::Render;

#[derive(Clone, Debug)]
pub struct AccordionItem {
    title: String,
    body: String,
}

#[derive(Clone, Debug, Default)]
pub struct Accordion {
    items: Vec<AccordionItem>,
    open_first: bool,
}

impl Accordion {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item(mut self, title: impl Into<String>, body: impl Into<String>) -> Self {
        self.items.push(AccordionItem {
            title: title.into(),
            body: body.into(),
        });
        self
    }

    pub fn open_first(mut self, open_first: bool) -> Self {
        self.open_first = open_first;
        self
    }
}

impl Render for Accordion {
    fn render(&self) -> String {
        let mut out = String::new();
        for (i, item) in self.items.iter().enumerate() {
            let open = self.open_first && i == 0;
            let button_class = if open {
                "tui-accordion__button tui-accordion__button--open"
            } else {
                "tui-accordion__button"
            };
            let collapse_class = if open {
                "tui-accordion__collapse show"
            } else {
                "tui-accordion__collapse"
            };
            out.push_str(&format!(
                "<div class=\"tui-accordion__item\"><button class=\"{button_class}\" type=\"button\">{}</button><div class=\"{collapse_class}\"><div class=\"tui-accordion__body\">{}</div></div></div>",
                item.title, item.body
            ));
        }
        out
    }
}
