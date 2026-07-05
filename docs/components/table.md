# Table

Data table built on iced 0.14's `table` widget.

## Install

```bash
iced-shadcn add table
```

## Usage

```rust
use iced::widget::table;
use ui::table::{table_view, text_cell};

table_view(
    [
        table::column(text("Invoice"), |row: Invoice| text_cell(&row.id)),
        table::column(text("Status"), |row: Invoice| text_cell(&row.status)),
        table::column(text("Amount"), |row: Invoice| text_cell(&row.amount)),
    ],
    rows.iter().cloned(),
)
```

`define a row type implementing `Clone` and pass column definitions with cell renderers.

## Helpers

| Function | Description |
|----------|-------------|
| `table_view` | Creates a styled table from columns and rows |
| `text_cell` | Renders a `&str` as a table cell |

## Example

```rust
#[derive(Clone)]
struct Invoice {
    id: &'static str,
    status: &'static str,
    amount: &'static str,
}

let rows = [
    Invoice { id: "INV001", status: "Paid", amount: "$250.00" },
    Invoice { id: "INV002", status: "Pending", amount: "$150.00" },
];

table_view(
    [
        table::column(text("Invoice"), |r: Invoice| text_cell(r.id)),
        table::column(text("Status"), |r: Invoice| text_cell(r.status)),
        table::column(text("Amount"), |r: Invoice| text_cell(r.amount)),
    ],
    rows.iter().cloned(),
)
```
