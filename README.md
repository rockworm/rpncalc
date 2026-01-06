# RPN Calculator

A terminal-based Reverse Polish Notation calculator built with Rust and Ratatui.

## Features

- **Arithmetic**: `+`, `-`, `*`, `/`, `^`, `%`
- **Math Functions**: `sqrt`, `inv` (1/x), `!` (factorial)
- **Trigonometry**: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`
- **Stack Operations**: `swap`, `drop`, `clear`, `undo`
- **Interactive TUI** with stack visualization and calculation history

## Usage

```bash
cargo run
```

### Controls
- Type numbers and press Enter to push to stack
- Type commands and press Enter to execute
- Single-character operators (`+`, `-`, `*`, `/`, `^`, `%`, `!`) execute immediately
- `q` to quit, `help` for command list, `Esc` to clear stack

### Example
```
Input: 5 4 +
Stack: [9]

Input: 3 *
Stack: [27]
```

## Installation

```bash
git clone <repo-url>
cd rpncalc
cargo build --release
```

## Testing

```bash
cargo test
```

## Requirements

- Rust 1.70+
- Terminal with Unicode support