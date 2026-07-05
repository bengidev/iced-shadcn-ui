use iced::widget::{column, container, text, Column};
use iced::{Background, Element, Theme};
use crate::ui::theme::palette;
use crate::ui::utils::border;

pub fn card<'a, Message: 'a>(title: &'a str, content: Element<'a, Message, Theme>) -> Element<'a, Message, Theme> {
    container(
        column![
            text(title).size(18.0),
            content,
        ]
        .spacing(8)
    )
    .padding(16)
    .style(|theme| {
        let p = palette(theme);
        container::Style {
            background: Some(Background::Color(p.card)),
            text_color: Some(p.card_foreground),
            border: border(p.border, 1.0, p.radius_lg),
            ..Default::default()
        }
    })
    .into()
}
