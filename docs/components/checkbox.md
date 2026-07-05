# Checkbox

Labeled checkbox with shadcn styling.

## Install

```bash
iced-shadcn add checkbox
```

## Usage

```rust
use ui::checkbox::checkbox;

checkbox("Accept terms and conditions", state.accepted, Message::TermsToggled)
```

| Parameter | Description |
|-----------|-------------|
| `label` | Label text |
| `checked` | Current checked state |
| `on_toggle` | Callback receiving the new `bool` |

## Example

```rust
checkbox("Remember me", state.remember, Message::RememberToggled)
```

Checked state uses `primary` fill with `primary_foreground` icon color.
