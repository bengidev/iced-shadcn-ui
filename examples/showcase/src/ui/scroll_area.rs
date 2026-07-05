use iced::widget::scrollable;
use iced::{Element, Length, Theme};

pub fn scroll_area<'a, Message: 'a>(
    content: Element<'a, Message, Theme>,
) -> Element<'a, Message, Theme> {
    scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
