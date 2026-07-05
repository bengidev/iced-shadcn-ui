use iced::widget::{table, text};
use iced::Element;

pub fn table_view<'a, Message: 'a, Row: Clone + 'a>(
    columns: impl IntoIterator<Item = table::Column<'a, 'a, Row, Message>>,
    rows: impl IntoIterator<Item = Row>,
) -> Element<'a, Message> {
    table::table(columns, rows).into()
}

pub fn text_cell<'a, Message: 'a>(value: &'a str) -> Element<'a, Message> {
    text(value).into()
}
