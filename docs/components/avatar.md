# Avatar

Circular avatar displaying initials.

## Install

```bash
iced-shadcn add avatar
```

## Usage

```rust
use ui::avatar::avatar;

avatar("JD")
```

## Example

```rust
row![
    avatar("JD"),
    avatar("AB"),
    avatar("CD"),
]
.spacing(8)
```

Renders a 32×32 circle with `muted` background. v0.1 displays initials only — image support may be added in future releases.
