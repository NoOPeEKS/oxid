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

pub struct BufferPosition {
    pub line: usize,
    pub character: usize,
}

#[derive(Clone, Debug)]
pub struct FileLine {
    pub content: String,
    pub length: usize,
}
