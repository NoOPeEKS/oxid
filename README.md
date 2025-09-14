# Oxid

A simple, vim-inspired terminal text editor built with Rust. This is a personal project to build a custom text editor and learn Rust in the process!

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/terminal%20based-4D4D4D?style=for-the-badge&logo=windows-terminal&logoColor=white)

## ✨ Features

### 🎯 **Modal Editing**
- **Normal Mode**: Navigate and manipulate text efficiently
- **Insert Mode**: Type and edit text naturally
- **Visual Mode**: Select and apply motions to your text
- **Command Mode**: Apply the main vim commands to your buffers.

### ⌨️ **Vim-Inspired Navigation**
- `h`, `j`, `k`, `l` - Move cursor left, down, up, right
- Familiar vim keybindings for natural text navigation

### 🖥️ **User Interface**
- **Status Bar**: Shows current mode and cursor position
- **Responsive Layout**: Adapts to terminal size

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
cargo run -- /path/to/file
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
| `v` | Enter visual mode |
| `p` | Paste previously yanked text |
| `Ctrl+u` | Scroll up |
| `Ctrl+d` | Scroll down |
| `Ctrl+c` | Quit editor |
| `Ctrl+s` | Save current file |
| `Shift+i` | Insert at start of line |
| `Shift+a` | Append at end of line |
| `:` | Enter command mode |

### Insert Mode
| Key | Action |
|-----|--------|
| `Esc` | Return to normal mode |
| `Backspace` | Delete character/merge lines |
| `Enter` | Insert a new line |
| `Any character` | Insert character |

### Visual Mode
| Key | Action |
|-----|--------|
| `Esc` | Return to normal mode |
| `v` | Return to normal mode |
| `h` | Move selection left |
| `j` | Move selection down |
| `k` | move selection up |
| `l` | Move selection right |
| `w` | Move selection one word forward |
| `b` | Move selection one word backwards |
| `e` | Move selection to the end of the word |
| `0` | Move selection to the start of the line |
| `$` | Move selection to the end of the line |
| `y` | Yank current selection to default register |

### Command Mode
| Command | Action |
|-----|--------|
| `:w` | Save current buffer file |
| `:wa` | Save all buffer files |
| `:q` | Quit current buffer |
| `:qa` | Quit all buffers|
| `:wqa` | Save and quit all buffers |
| `:e <file_path>` | Open or create a new file |
| `:bn` | Move to next buffer |
| `:bp` | Move to previous buffer |
| `:<line_number>` | Move to specified line on current buffer |

## 📋 Planned Features

> **Note**: The following features are planned or currently under development. Some are top-priority core functionalities and others are features that will be implemented in future releases.

### 🔧 **Text Operations**
- [ ] **Copy/Paste Support**
  - [ ] System clipboard integration
- [ ] **Advanced Editing**
  - [ ] Undo/Redo functionality

### 📁 **File Management**
- [ ] **File Operations**
  - [ ] Save as
  - [x] Multi-file editing

### 🎨 **User Interface Enhancements**
- [ ] **Syntax Highlighting**
  - [ ] Color scheme

### **Command Mode**
- [x] **File Management**
    - [x] Save (:w)
    - [x] Save All (:wa)
    - [x] Quit (:q)
    - [x] Quit All (:qa)
    - [x] Save and Quit All (:wqa)
    - [x] Open new buffer (:e <file>)
    - [x] Go to next buffer (:bn)
    - [x] Go to prev buffer (:bp)
- [ ] **Navigation**
  - [x] Go to line number (`:line`)
  - [ ] Search

### 🧩 **Language Server Protocol**
- [ ] **LSP Client**
  - [x] Autocompletion
  - [x] Hovering
  - [x] Diagnostics
- [ ] **Editor UI Integration**
  - [x] Auto-completion
  - [x] Hovering
  - [ ] Diagnostics

## 🤝 Contributing

Contributions are welcome! Whether you want to:
- Fix bugs
- Add new features
- Improve documentation
- Optimize performance

Please feel free to open issues and submit pull requests.

## 📄 License

This project is licensed under the AGPLv3 License - see the [LICENSE](LICENSE) file for details.
