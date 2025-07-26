#[derive(PartialEq)]
pub enum CharType {
    Word,
    Punctuation,
    Whitespace,
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
