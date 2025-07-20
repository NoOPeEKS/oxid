# Oxid

A simple Terminal Text Editor built in Rust for learning purposes.

# TODOS
- Separate event handling into a thread to make it non-blocking.
- Refactor code into more modular and extensible way.
- Insert Mode:
    * Add characters to an existing line and move the others if necessary.
    * Backspace removes characters.
    * If all characters of a new line are removed using backspace, delete the line.
