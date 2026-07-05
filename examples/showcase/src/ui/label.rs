use iced::widget::text;
use iced::{Element, Theme};
use crate::ui::theme::palette;

pub fn label<'a, Message: 'a>(content: &'a str) -> Element<'a, Message, Theme> {
    text(content)
        .size(14.0)
        .style(|theme| text::Style {
            color: Some(palette(theme).foreground),
        })
        .into()
}
