# Oxid

A simple Terminal Text Editor built in Rust for learning purposes.

# TODOS
- Fix bug when you are at last char of line it doesn't let you move left.
- Fix performance issues when inserting new characters.
- Insert Mode:
    * Backspace removes characters.
# BUGS
- Whenever you come out from insert mode, cursor should back up a position. Other wise it lets the cursor stay one tile ahead of where it's supposed to go.
- Deleting characters is tricky, lots of bugs happen when removing last and first char of a string or when string becomes empty.
- Cursor when inserting, inserts before cursor. Cursor when deleting, deletes what's inside of cursor.
