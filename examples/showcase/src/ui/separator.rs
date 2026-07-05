use iced::widget::{row, rule};
use iced::{Element, Length, Theme};
use crate::ui::theme::palette;

pub fn separator<'a, Message: 'a>() -> Element<'a, Message, Theme> {
    row![rule::horizontal(1).style(|theme| {
        let p = palette(theme);
        rule::Style {
            color: p.border,
            radius: iced::border::Radius::default(),
            fill_mode: rule::FillMode::Full,
            snap: true,
        }
    })]
    .width(Length::Fill)
    .into()
}
