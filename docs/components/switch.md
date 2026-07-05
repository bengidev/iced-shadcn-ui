# Switch

Toggle switch for boolean settings.

## Install

```bash
iced-shadcn add switch
```

## Usage

```rust
use ui::switch::switch;

switch(state.notifications, "Email notifications", Message::NotificationsToggled)
```

| Parameter | Description |
|-----------|-------------|
| `enabled` | Current on/off state |
| `label` | Label text |
| `on_toggle` | Callback receiving the new `bool` |

## Example

```rust
switch(state.dark_mode, "Dark mode", Message::ToggleTheme)
```

Track uses `primary` when on and `input` when off; thumb uses `background`.
