# Oxid

A simple Terminal Text Editor built in Rust for learning purposes, inspired by Vim.

# TODOS
- Fix performance issues when inserting new characters.
- Implement test cases.
- Refactor file lines into structs for more readability.
- Implement some sort of viewport to scroll long files (right now it can only show what the screen allows).
- Insert Mode:
    * Using backspace on an empty line should delete the line and move the cursor upwards unless its the first line.
    * Ctrl + hjkl should allow to move cursor in insert mode.
- Vim bindings:
    * 'w' should place cursor in next word.
    * 'b' should place cursor in previous word.
    * 'dd' should delete current line.
    * Implement some sort of rudimentary concatenation of motions.
