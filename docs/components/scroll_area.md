# Scroll area

Scrollable viewport for overflow content.

## Install

```bash
iced-shadcn add scroll-area
```

CLI name is `scroll-area`; the module file is `scroll_area.rs`.

## Usage

```rust
use ui::scroll_area::scroll_area;

scroll_area(content_element)
```

## Example

```rust
let content = column((0..20).map(|i| text(format!("Row {i}")).into()).collect::<Vec<_>>())
    .spacing(8)
    .width(Length::Fill);

column![
    text("Scroll area"),
    scroll_area(content.into()),
]
.height(Length::Fixed(160.0))
```

Wraps iced's `scrollable` widget at fill width and height.
