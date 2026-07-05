# Separator

Horizontal divider line between content sections.

## Install

```bash
iced-shadcn add separator
```

## Usage

```rust
use ui::separator::separator;

separator::<Message>()
```

## Example

```rust
column![
    text("Section above"),
    separator::<()>(),
    text("Section below"),
]
.spacing(12)
```

Uses the `border` token from the active palette.
