# Skeleton

Placeholder block for loading states.

## Install

```bash
iced-shadcn add skeleton
```

## Usage

```rust
use ui::skeleton::skeleton;

skeleton(240.0, 16.0)  // width, height in pixels
```

## Example

```rust
column![
    skeleton(240.0, 16.0),
    skeleton(180.0, 16.0),
]
.spacing(8)
```

Rendered with the `muted` background token and rounded corners from the theme.
