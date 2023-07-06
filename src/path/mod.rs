use crate::token::Token;

pub struct Path();

impl Path {
    pub fn new() -> Self {
        Path()
    }
    pub fn empty_path() -> Self {
        Path()
    }
    pub fn absolute_root() -> Self {
        Path()
    }
    pub fn is_absolute(&self) -> bool {
        false
    }
    pub fn is_namespaced(&self) -> bool {
        false
    }
    pub fn is_empty(&self) -> bool {
        false
    }
    pub fn append_property(&self, token: &Token) -> &Self {
        &self
    }
    pub fn append_token(&self, token: &Token) -> &Self {
        &self
    }
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        Path()
    }
}

impl Into<String> for Path {
    fn into(self) -> String {
        String::from("")
    }
}
