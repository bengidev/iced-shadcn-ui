# Badge

Inline status label for counts, tags, and states.

## Install

```bash
iced-shadcn add badge
```

## Usage

```rust
use ui::badge::{badge, BadgeVariant};

badge("New", BadgeVariant::Default)
```

## Variants

| Variant | Description |
|---------|-------------|
| `Default` | Primary fill |
| `Secondary` | Secondary fill |
| `Destructive` | Error/warning state |
| `Outline` | Bordered |

## Example

```rust
row![
    badge("Default", BadgeVariant::Default),
    badge("Secondary", BadgeVariant::Secondary),
    badge("Destructive", BadgeVariant::Destructive),
    badge("Outline", BadgeVariant::Outline),
]
.spacing(8)
```
