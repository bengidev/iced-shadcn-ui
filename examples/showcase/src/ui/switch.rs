use iced::widget::toggler;
use iced::{Element, Theme};
use crate::ui::theme::palette;

pub fn switch<'a, Message: Clone + 'a>(
    enabled: bool,
    label: &'a str,
    on_toggle: impl Fn(bool) -> Message + 'a,
) -> Element<'a, Message, Theme> {
    toggler(enabled)
        .label(label)
        .on_toggle(on_toggle)
        .style(move |theme, status| {
            let p = palette(theme);
            let mut style = toggler::default(theme, status);
            style.background = iced::Background::Color(if enabled {
                p.primary
            } else {
                p.input
            });
            style.background_border_width = 1.0;
            style.background_border_color = p.border;
            style.foreground = iced::Background::Color(p.background);
            style
        })
        .into()
}
