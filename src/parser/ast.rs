pub struct Parser {
    tokens: Vec<Token>;
    cursor: usize;
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0;
        }
    }
}