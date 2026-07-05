use crate::ui;
use iced::widget::{column, row, text, Column};
use iced::Element;

pub fn view() -> Element<'static, ()> {
    let card_content = column![
        text("Card content with supporting text."),
        ui::separator::separator::<()>(),
        row![
            ui::avatar::avatar("JD"),
            ui::avatar::avatar("AB"),
            ui::avatar::avatar("CD"),
        ]
        .spacing(8),
    ]
    .spacing(12);

    let scroll_content = column((0..20).map(|i| text(format!("Scrollable row {i}")).into()).collect::<Vec<_>>())
        .spacing(8)
        .width(iced::Length::Fill);

    Column::new()
        .spacing(24)
        .push(text("Layout").size(24))
        .push(ui::card::card("Card title", card_content.into()))
        .push(
            column![
                text("Scroll area").size(18),
                ui::scroll_area::scroll_area(scroll_content.into()),
            ]
            .spacing(8)
            .height(iced::Length::Fixed(160.0)),
        )
        .into()
}
