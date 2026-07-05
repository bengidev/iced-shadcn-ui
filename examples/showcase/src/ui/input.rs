use iced::widget::text_input;
use iced::{Element, Theme};
use crate::ui::theme::palette;
use crate::ui::utils::border;

pub fn input<'a, Message: Clone + 'a>(
    placeholder: &'a str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message, Theme> {
    text_input(placeholder, value)
        .on_input(on_input)
        .padding(10)
        .style(|theme, status| {
            let p = palette(theme);
            let base = text_input::Style {
                background: iced::Background::Color(p.background),
                border: border(p.input, 1.0, p.radius_md),
                icon: p.muted_foreground,
                placeholder: p.muted_foreground,
                value: p.foreground,
                selection: p.primary,
            };
            match status {
                text_input::Status::Focused { .. } => text_input::Style {
                    border: border(p.ring, 2.0, p.radius_md),
                    ..base
                },
                _ => base,
            }
        })
        .into()
}
