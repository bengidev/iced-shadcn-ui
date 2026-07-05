# Input

Styled text input with focus ring.

## Install

```bash
iced-shadcn add input
```

## Usage

```rust
use ui::input::input;

input("you@example.com", &state.email, Message::EmailChanged)
```

| Parameter | Description |
|-----------|-------------|
| `placeholder` | Placeholder text |
| `value` | Current value (`&str`) |
| `on_input` | Callback receiving the new string |

## Example

```rust
column![
    label("Email"),
    input("you@example.com", &state.email, Message::EmailChanged),
]
.spacing(6)
```

Focus state applies a `ring`-colored border. Placeholder text uses `muted_foreground`.
