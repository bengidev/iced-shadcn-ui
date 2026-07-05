use crate::ui;
use iced::widget::{column, text, Column};
use iced::Element;

pub fn view() -> Element<'static, ()> {
    Column::new()
        .spacing(24)
        .push(text("Inputs").size(24))
        .push(
            column![
                ui::label::label("Email"),
                ui::input::input("you@example.com", "you@example.com", |_| ()),
            ]
            .spacing(6),
        )
        .push(ui::checkbox::checkbox("Accept terms and conditions", true, |_| ()))
        .push(ui::switch::switch(true, "Email notifications", |_| ()))
        .into()
}
