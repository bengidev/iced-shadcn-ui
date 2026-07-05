# Card

Bordered content container with title and body.

## Install

```bash
iced-shadcn add card
```

## Usage

```rust
use ui::card::card;

card("Card title", content_element)
```

| Parameter | Description |
|-----------|-------------|
| `title` | Header text |
| `content` | Body `Element` |

## Example

```rust
let content = column![
    text("Card content with supporting text."),
    separator::<()>(),
    row![avatar("JD"), avatar("AB")].spacing(8),
]
.spacing(12);

card("Card title", content.into())
```

Uses `card` and `card_foreground` tokens with `radius_lg` border.
