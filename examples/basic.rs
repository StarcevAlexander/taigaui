use taigaui::{
    Accordion, Alert, AlertKind, Button, ButtonKind, Card, Checkbox, Input, ProgressBar,
    ProgressRing, Radio, Render, Slider, Tabs, Tag, TagColor, Textarea, Toggle, Tooltip, css,
};

fn main() {
    let mut page = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>taigaui gallery</title><style>",
    );
    page.push_str(&css::stylesheet());
    page.push_str("body { margin: 0; background: var(--tui-bg); }");
    page.push_str(".demo { padding: 32px 40px 80px; display: grid; gap: 20px; max-width: 720px; }");
    page.push_str(".demo-row { display: flex; gap: 12px; flex-wrap: wrap; align-items: center; }");
    page.push_str("</style></head><body><div class=\"demo\">");

    page.push_str("<p class=\"tui-input__label\" style=\"text-transform: uppercase; letter-spacing: 0.5px;\">Buttons</p><div class=\"demo-row\">");
    page.push_str(&Button::new("Primary").render());
    page.push_str(
        &Button::new("Secondary")
            .kind(ButtonKind::Secondary)
            .render(),
    );
    page.push_str(&Button::new("Flat").kind(ButtonKind::Flat).render());
    page.push_str(&Button::new("Delete").kind(ButtonKind::Danger).render());
    page.push_str(&Button::new("+").round(true).render());
    page.push_str(&Button::new("Disabled").disabled(true).render());
    page.push_str("</div>");

    page.push_str("<p class=\"tui-input__label\" style=\"text-transform: uppercase; letter-spacing: 0.5px;\">Tags</p><div class=\"demo-row\">");
    page.push_str(&Tag::new("neutral").render());
    page.push_str(&Tag::new("primary").color(TagColor::Primary).render());
    page.push_str(&Tag::new("success").color(TagColor::Success).render());
    page.push_str(&Tag::new("warning").color(TagColor::Warning).render());
    page.push_str(&Tag::new("danger").color(TagColor::Danger).render());
    page.push_str("</div>");

    page.push_str(
        &Card::new()
            .title("Taiga UI for Rust")
            .body("Typed builders rendering framework-agnostic HTML in the Taiga design style.")
            .render(),
    );

    page.push_str("<p class=\"tui-input__label\" style=\"text-transform: uppercase; letter-spacing: 0.5px;\">Inputs</p>");
    page.push_str(
        &Input::new("Email", "name@example.com")
            .name("email")
            .render(),
    );
    page.push_str(&Textarea::new("Comment", "Type here...").rows(3).render());

    page.push_str("<div class=\"demo-row\">");
    page.push_str(&Checkbox::new("Remember me").checked(true).render());
    page.push_str(&Radio::new("size", "m", "Medium").checked(true).render());
    page.push_str(&Toggle::new("Dark mode").checked(true).render());
    page.push_str("</div><div class=\"demo-row\">");
    page.push_str(&Slider::new("volume").value(35.0).render());
    page.push_str(
        &Tooltip::new(
            Button::new("Hover me").kind(ButtonKind::Flat).render(),
            "Pure CSS tooltip",
        )
        .render(),
    );
    page.push_str("</div>");

    page.push_str("<p class=\"tui-input__label\" style=\"text-transform: uppercase; letter-spacing: 0.5px;\">Tabs</p>");
    page.push_str(
        &Tabs::new()
            .tab("Overview", true)
            .tab("Specs", false)
            .tab("Reviews", false)
            .render(),
    );

    page.push_str("<p class=\"tui-input__label\" style=\"text-transform: uppercase; letter-spacing: 0.5px;\">Alerts</p>");
    page.push_str(&Alert::new("Neutral notice").render());
    page.push_str(
        &Alert::new("Everything works!")
            .kind(AlertKind::Success)
            .render(),
    );
    page.push_str(
        &Alert::new("Careful there.")
            .kind(AlertKind::Warning)
            .render(),
    );
    page.push_str(
        &Alert::new("Something broke.")
            .kind(AlertKind::Danger)
            .render(),
    );

    page.push_str("<p class=\"tui-input__label\" style=\"text-transform: uppercase; letter-spacing: 0.5px;\">Progress</p><div class=\"demo-row\">");
    page.push_str(&ProgressBar::new(65).render());
    page.push_str(&ProgressRing::new().render());
    page.push_str("</div>");

    page.push_str("<p class=\"tui-input__label\" style=\"text-transform: uppercase; letter-spacing: 0.5px;\">Accordion</p>");
    page.push_str(
        &Accordion::new()
            .item(
                "What is taigaui?",
                "A Taiga UI styled component set for Rust.",
            )
            .item(
                "Do I need JavaScript?",
                "No. Native interactive elements only.",
            )
            .open_first(true)
            .render(),
    );

    page.push_str("</div></body></html>");
    println!("{page}");
}
