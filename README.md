# Oxid

A simple Terminal Text Editor built in Rust for learning purposes, inspired by Vim.

# TODOS
- Add cli argument handling for choosing file.
- Fix performance issues when inserting new characters.
- Implement test cases.
- Implement some sort of viewport to scroll long files (right now it can only show what the screen allows).
- Insert Mode:
    * Ctrl + hjkl should allow to move cursor in insert mode.
- Vim bindings:
    * 'w' should place cursor in next word.
    * 'b' should place cursor in previous word.
    * 'dd' should delete current line.
    * Implement some sort of rudimentary concatenation of motions.
