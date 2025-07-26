# Oxid

A simple, vim-inspired terminal text editor built with Rust. This is a personal project to build a custom text editor and learn Rust in the process!

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/terminal%20based-4D4D4D?style=for-the-badge&logo=windows-terminal&logoColor=white)

## ✨ Features

### 🎯 **Modal Editing**
- **Normal Mode**: Navigate and manipulate text efficiently
- **Insert Mode**: Type and edit text naturally

### ⌨️ **Vim-Inspired Navigation**
- `h`, `j`, `k`, `l` - Move cursor left, down, up, right
- Familiar vim keybindings for natural text navigation
- Smart cursor positioning that respects line boundaries

### 📜 **Viewport Management**
- **Smart Scrolling**: Automatic viewport adjustment
- **Line Numbers**: Clean, minimal line number display

### 🖥️ **User Interface**
- **Status Bar**: Shows current mode and cursor position
- **Responsive Layout**: Adapts to terminal size

### ⚡ **Performance**
- **Efficient Rendering**: Only renders visible lines
- **Asynchronous Events**: Non-blocking input handling

## 🚀 Quick Start

### Prerequisites
- Rust 1.88
- A terminal emulator

### Installation & Usage

1. **Clone the repository**
```bash
git clone git@github.com:NoOPeEKS/oxid.git
cd oxid
```

2. **Build the project**
```bash
cargo build --release
```

3. **Run the editor**
```bash
cargo run
```
As of now, the editor will either open an existing file or create a new one. It does not currently support multiple buffers nor opening directories.

## 🎮 Key Bindings

### Normal Mode
| Key | Action |
|-----|--------|
| `h` | Move cursor left |
| `j` | Move cursor down |
| `k` | Move cursor up |
| `l` | Move cursor right |
| `w` | Move one word forward |
| `b` | Move one word backwards |
| `e` | Move to the end of the word |
| `0` | Move to the start of the line |
| `$` | Move to the end of the line |
| `o` | Insert line below and enter insert mode |
| `i` | Enter insert mode |
| `Ctrl+u` | Scroll up |
| `Ctrl+d` | Scroll down |
| `Ctrl+c` | Quit editor |
| `Ctrl+s` | Save current file |
| `Shift+i` | Insert at start of line |
| `Shift+a` | Append at end of line |

### Insert Mode
| Key | Action |
|-----|--------|
| `Esc` | Return to normal mode |
| `Backspace` | Delete character/merge lines |
| `Enter` | Insert a new line |
| `Any character` | Insert character |

## 📋 Features in Progress

> **Note**: The following features are planned or currently under development. Some are top-priority core functionalities and others are features that will be implemented in future releases.

### 🔧 **Text Operations**
- [ ] **Copy/Paste Support**
  - [ ] Visual selection mode [Core]
  - [ ] Yank (copy) operations[Core]
  - [ ] Paste operations [Core]
  - [ ] System clipboard integration [Core]

- [ ] **Advanced Editing**
  - [x] Word-based navigation (`w`, `b`, `e`) [Core]
  - [ ] Undo/Redo functionality

### 📁 **File Management**
- [ ] **File Operations**
  - [x] Open arbitrary file (command line argument) [Core]
  - [x] Save current file [Core]
  - [ ] Save as
  - [x] New file creation
  - [ ] Multi-file editing

### 🎨 **User Interface Enhancements**
- [ ] **Syntax Highlighting**
  - [ ] Color scheme

- [ ] **Visual Improvements**
  - [ ] UI enhancement with colors, borders... [Core]


### 🔍 **Navigation**
  - [ ] Go to line number (`:line`)


### 🧩 **Developer Tools**
- [ ] **Integration Features**
  - [ ] LSP (Language Server Protocol) support

## 🤝 Contributing

Contributions are welcome! Whether you want to:
- Fix bugs
- Add new features
- Improve documentation
- Optimize performance

Please feel free to open issues and submit pull requests.

## 📄 License

This project is licensed under the AGPLv3 License - see the [LICENSE](LICENSE) file for details.
