# reqtuitui

A beautiful and efficient Terminal UI (TUI) for making HTTP requests, built with Rust and Ratatui.

## Features

- **Interactive TUI:** Fast, keyboard-driven interface for crafting and sending HTTP requests.
- **cURL Support:** Import and parse cURL commands directly into the UI.
- **JSON Support:** Built-in JSON formatting and request bodies.
- **State Persistence:** Automatically saves your session state and requests history.

## Installation

### Using Homebrew (macOS/Linux)

You can install `reqtuitui` via Homebrew using our custom tap:

```bash
brew install planktonsoft/homebrew-reqtuitui/reqtuitui
```

### Using Cargo

If you have Rust installed, you can easily install `reqtuitui` using `cargo`:

```bash
cargo install reqtuitui
```

### From Source

```bash
git clone https://github.com/planktonsoft/reqtuitui.git
cd reqtuitui
cargo build --release
```
The binary will be located in `target/release/reqtuitui`.

## Usage

Simply run the command in your terminal:

```bash
reqtuitui
```

### Keyboard Shortcuts

`reqtuitui` is heavily keyboard-driven. Here are the primary shortcuts to navigate and use the application efficiently:

#### Global Shortcuts
- **`Tab`**: Switch focus between panes (Sidebar -> URL Bar -> Headers -> Body)
- **`Ctrl + N`**: Create a new request
- **`Ctrl + F`**: Create a new folder
- **`Ctrl + S`**: Save the current request/collection
- **`Ctrl + E`**: Open Environment variables popup
- **`Ctrl + O`**: Import cURL command
- **`Ctrl + Y`**: Cycle HTTP method (GET, POST, PUT, DELETE, PATCH)
- **`Ctrl + R`**: Rename selected folder or request
- **`PageUp` / `PageDown`**: Scroll response view up/down (also `Ctrl+U`/`Ctrl+D`)
- **`Esc`**: Close popups or cancel actions

#### Sidebar (List) Controls
When the sidebar is focused:
- **`Up` / `k`**: Move selection up
- **`Down` / `j`**: Move selection down
- **`Enter`**: Toggle folder expansion or execute the selected request
- **`e`**: Edit the URL of the selected request
- **`Delete` / `Backspace`**: Delete the selected item
- **`q`**: Quit application


## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is licensed under the terms of the MIT license.
