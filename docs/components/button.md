# Button

Primary action control with shadcn variants and sizes.

## Install

```bash
iced-shadcn add button
```

## Usage

```rust
use ui::button::{button, ButtonVariant, ButtonSize};

button("Save")
    .variant(ButtonVariant::Default)
    .on_press(Message::Save)
    .into_button()
```

## Variants

| Variant | Description |
|---------|-------------|
| `Default` | Filled primary button |
| `Destructive` | Destructive action |
| `Outline` | Bordered, transparent background |
| `Secondary` | Secondary fill |
| `Ghost` | Transparent with hover accent |
| `Link` | Text-only link style |

## Sizes

| Size | Description |
|------|-------------|
| `Default` | Standard padding |
| `Sm` | Compact |
| `Lg` | Large |
| `Icon` | Square icon button |

## Example

```rust
row![
    button("Default").variant(ButtonVariant::Default).into_button(),
    button("Outline").variant(ButtonVariant::Outline).into_button(),
    button("Ghost").variant(ButtonVariant::Ghost).into_button(),
]
.spacing(8)
```
