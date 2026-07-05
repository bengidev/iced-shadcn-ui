use crate::ui::{self, badge::BadgeVariant, button::ButtonVariant};
use iced::widget::{column, row, text, Column};
use iced::Element;
use lucide_icons::Icon;

pub fn view() -> Element<'static, ()> {
    let variants = column![
        text("Variants").size(18),
        row![
            ui::button::button("Default")
                .variant(ButtonVariant::Default)
                .into_button(),
            ui::button::button("Destructive")
                .variant(ButtonVariant::Destructive)
                .into_button(),
            ui::button::button("Outline")
                .variant(ButtonVariant::Outline)
                .into_button(),
        ]
        .spacing(8),
        row![
            ui::button::button("Secondary")
                .variant(ButtonVariant::Secondary)
                .into_button(),
            ui::button::button("Ghost")
                .variant(ButtonVariant::Ghost)
                .into_button(),
            ui::button::button("Link")
                .variant(ButtonVariant::Link)
                .into_button(),
        ]
        .spacing(8),
    ]
    .spacing(12);

    let icons = column![
        text("Lucide icons").size(18),
        row![
            ui::button::icon_button(Icon::Plus).into_button(),
            ui::button::icon_button(Icon::Trash2)
                .variant(ButtonVariant::Destructive)
                .into_button(),
            ui::button::button_with_icon(Icon::Download, "Download").into_button(),
            ui::button::button_with_icon(Icon::Search, "Search")
                .variant(ButtonVariant::Outline)
                .into_button(),
        ]
        .spacing(8),
    ]
    .spacing(12);

    let badges = column![
        text("Badges").size(18),
        row![
            ui::badge::badge("Default", BadgeVariant::Default),
            ui::badge::badge("Secondary", BadgeVariant::Secondary),
            ui::badge::badge("Destructive", BadgeVariant::Destructive),
            ui::badge::badge("Outline", BadgeVariant::Outline),
        ]
        .spacing(8),
    ]
    .spacing(12);

    Column::new()
        .spacing(24)
        .push(text("Buttons").size(24))
        .push(variants)
        .push(icons)
        .push(badges)
        .into()
}
