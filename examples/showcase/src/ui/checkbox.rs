use iced::widget::checkbox::{self, primary};
use iced::widget::checkbox as iced_checkbox;
use iced::{Element, Theme};
use crate::ui::theme::palette;

pub fn checkbox<'a, Message: Clone + 'a>(
    label: &'a str,
    checked: bool,
    on_toggle: impl Fn(bool) -> Message + 'a,
) -> Element<'a, Message, Theme> {
    iced_checkbox(checked)
        .label(label)
        .on_toggle(on_toggle)
        .style(|theme, status| {
            let p = palette(theme);
            let _base = primary(theme, status);
            checkbox::Style {
                icon_color: p.primary_foreground,
                background: iced::Background::Color(p.background),
                border: iced::Border {
                    color: p.primary,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                text_color: Some(p.foreground),
                .._base
            }
        })
        .into()
}
