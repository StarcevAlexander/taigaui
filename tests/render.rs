use taigaui::{
    Accordion, Alert, AlertKind, Button, ButtonKind, Card, Checkbox, Input, ProgressBar,
    ProgressRing, Radio, Render, Slider, Tabs, Tag, TagColor, Textarea, Toggle, Tooltip, css,
};

#[test]
fn button_renders_default_primary() {
    let html = Button::new("Go").render();
    assert!(html.contains("tui-btn"));
    assert!(!html.contains("tui-btn--"));
    assert!(html.contains(">Go</button>"));
}

#[test]
fn button_kinds_map_to_classes() {
    assert!(
        Button::new("x")
            .kind(ButtonKind::Secondary)
            .render()
            .contains("tui-btn--secondary")
    );
    assert!(
        Button::new("x")
            .kind(ButtonKind::Flat)
            .render()
            .contains("tui-btn--flat")
    );
    assert!(
        Button::new("x")
            .kind(ButtonKind::Danger)
            .render()
            .contains("tui-btn--danger")
    );
}

#[test]
fn round_button_adds_class() {
    assert!(
        Button::new("+")
            .round(true)
            .render()
            .contains("tui-btn--round")
    );
}

#[test]
fn disabled_button_carries_attribute() {
    assert!(
        Button::new("x")
            .disabled(true)
            .render()
            .contains(" disabled")
    );
}

#[test]
fn tag_renders_colors() {
    let neutral = Tag::new("n").render();
    assert!(neutral.contains("tui-tag"));
    assert!(!neutral.contains("tui-tag--"));
    let danger = Tag::new("d").color(TagColor::Danger).render();
    assert!(danger.contains("tui-tag--danger"));
}

#[test]
fn card_renders_title_and_body() {
    let html = Card::new().title("T").body("B").render();
    assert!(html.contains("tui-card__title\">T</h3>"));
    assert!(html.contains("tui-card__body\">B</div>"));
}

#[test]
fn input_renders_label_placeholder_and_name() {
    let html = Input::new("Email", "you@x.io").name("email").render();
    assert!(html.contains("tui-input__label\">Email</label>"));
    assert!(html.contains("placeholder=\"you@x.io\""));
    assert!(html.contains("name=\"email\""));
}

#[test]
fn textarea_renders_rows() {
    let html = Textarea::new("Comment", "type").rows(6).render();
    assert!(html.contains("tui-textarea"));
    assert!(html.contains("rows=\"6\""));
}

#[test]
fn checkbox_checked_state() {
    assert!(
        Checkbox::new("a")
            .checked(true)
            .render()
            .contains(" checked")
    );
    assert!(!Checkbox::new("a").render().contains(" checked"));
}

#[test]
fn radio_renders_group() {
    let html = Radio::new("plan", "pro", "Pro").checked(true).render();
    assert!(html.contains("name=\"plan\""));
    assert!(html.contains("value=\"pro\""));
    assert!(html.contains(" checked"));
}

#[test]
fn toggle_renders_track_thumb() {
    let html = Toggle::new("Dark").checked(true).render();
    assert!(html.contains("tui-toggle__track"));
    assert!(html.contains("tui-toggle__thumb"));
    assert!(html.contains(" checked"));
}

#[test]
fn slider_renders_fill() {
    let html = Slider::new("v").range(0.0, 10.0).value(4.0).render();
    assert!(html.contains("type=\"range\""));
    assert!(html.contains("--tui-slider-value: 40%"));
}

#[test]
fn tabs_mark_active() {
    let html = Tabs::new().tab("One", true).tab("Two", false).render();
    assert!(html.contains("tui-tabs__item--active"));
    assert!(html.contains("aria-selected=\"true\""));
    assert!(html.contains("aria-selected=\"false\""));
}

#[test]
fn alert_kinds() {
    assert!(!Alert::new("x").render().contains("tui-alert--"));
    assert!(
        Alert::new("x")
            .kind(AlertKind::Success)
            .render()
            .contains("tui-alert--success")
    );
    assert!(
        Alert::new("x")
            .kind(AlertKind::Warning)
            .render()
            .contains("tui-alert--warning")
    );
    assert!(
        Alert::new("x")
            .kind(AlertKind::Danger)
            .render()
            .contains("tui-alert--danger")
    );
}

#[test]
fn progress_bar_renders_width() {
    let html = ProgressBar::new(65).render();
    assert!(html.contains("aria-valuenow=\"65\""));
    assert!(html.contains("width: 65%;"));
}

#[test]
fn progress_ring_renders_status() {
    assert!(ProgressRing::new().render().contains("role=\"status\""));
}

#[test]
fn accordion_opens_first() {
    let html = Accordion::new()
        .item("Q1", "A1")
        .item("Q2", "A2")
        .open_first(true)
        .render();
    assert!(html.contains("tui-accordion__button--open"));
    assert!(html.contains("collapse show"));
    assert!(html.contains("collapse"));
}

#[test]
fn tooltip_wraps_bubble() {
    let html = Tooltip::new(Button::new("x").render(), "tip").render();
    assert!(html.contains("tui-tooltip__bubble\">tip</span>"));
}

#[test]
fn stylesheet_covers_all_components() {
    let sheet = css::stylesheet();
    for class in [
        ".tui-btn",
        ".tui-btn--secondary",
        ".tui-btn--flat",
        ".tui-btn--danger",
        ".tui-btn--round",
        ".tui-tag",
        ".tui-tag--primary",
        ".tui-card",
        ".tui-input__field",
        ".tui-textarea",
        ".tui-checkbox",
        ".tui-radio",
        ".tui-toggle",
        ".tui-slider",
        ".tui-tabs__item--active",
        ".tui-alert--success",
        ".tui-progress__bar",
        ".tui-progress-ring",
        ".tui-accordion__item",
        ".tui-tooltip__bubble",
    ] {
        assert!(sheet.contains(class), "stylesheet missing {class}");
    }
}
