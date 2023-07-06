#[derive(Clone, Hash)]
pub struct Token(String);

impl Token {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        Token { 0: s.to_owned() }
    }
}
impl From<String> for Token {
    fn from(s: String) -> Self {
        Token { 0: s }
    }
}
