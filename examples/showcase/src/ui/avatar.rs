use iced::widget::{container, text};
use iced::{alignment, Background, Element, Length, Theme};
use crate::ui::theme::palette;

pub fn avatar<'a, Message: 'a>(initials: &'a str) -> Element<'a, Message, Theme> {
    container(
        text(initials)
            .size(12.0)
            .align_x(alignment::Horizontal::Center),
    )
    .width(Length::Fixed(32.0))
    .height(Length::Fixed(32.0))
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(|theme| {
        let p = palette(theme);
        container::Style {
            background: Some(Background::Color(p.muted)),
            text_color: Some(p.foreground),
            border: iced::Border {
                radius: 999.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .into()
}
