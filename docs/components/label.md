# Label

Form field label with foreground text styling.

## Install

```bash
iced-shadcn add label
```

## Usage

```rust
use ui::label::label;

label("Email")
```

## Example

Pair with `input` for accessible form layouts:

```rust
column![
    label("Email"),
    input("you@example.com", &state.email, Message::EmailChanged),
]
.spacing(6)
```
