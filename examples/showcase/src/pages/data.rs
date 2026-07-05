use crate::ui;
use iced::widget::{column, table, text, Column};
use iced::Element;

#[derive(Clone)]
struct Invoice {
    id: &'static str,
    status: &'static str,
    amount: &'static str,
}

pub fn view() -> Element<'static, ()> {
    let rows = [
        Invoice {
            id: "INV001",
            status: "Paid",
            amount: "$250.00",
        },
        Invoice {
            id: "INV002",
            status: "Pending",
            amount: "$150.00",
        },
        Invoice {
            id: "INV003",
            status: "Overdue",
            amount: "$350.00",
        },
    ];

    let table = ui::table::table_view(
        [
            table::column(text("Invoice"), |row: Invoice| ui::table::text_cell(&row.id)),
            table::column(text("Status"), |row: Invoice| ui::table::text_cell(&row.status)),
            table::column(text("Amount"), |row: Invoice| ui::table::text_cell(&row.amount)),
        ],
        rows.iter().cloned(),
    );

    Column::new()
        .spacing(24)
        .push(text("Data").size(24))
        .push(table)
        .push(
            column![
                text("Progress").size(18),
                ui::progress::progress(62.0),
            ]
            .spacing(8),
        )
        .push(
            column![
                text("Skeleton").size(18),
                ui::skeleton::skeleton(240.0, 16.0),
                ui::skeleton::skeleton(180.0, 16.0),
            ]
            .spacing(8),
        )
        .into()
}
