#[derive(PartialEq)]
pub enum CharType {
    Word,
    Punctuation,
    Whitespace,
}

impl CharType {
    pub fn from_char(c: char) -> Self {
        if c.is_whitespace() {
            Self::Whitespace
        } else if c.is_alphanumeric() || c == '_' {
            Self::Word
        } else {
            Self::Punctuation
        }
    }
}

#[derive(Clone, Debug)]
pub struct BufferPosition {
    pub line: usize,
    pub character: usize,
}

#[derive(Clone, Debug)]
pub struct FileLine {
    pub content: String,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub start: BufferPosition,
    pub end: BufferPosition,
}
