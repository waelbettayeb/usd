trait Index {
    fn new() -> Self;
    fn from(value: u32) -> Self;
}

macro_rules! impl_index {
    ($struct:ident) => {
        impl Index for $struct {
            fn new() -> Self {
                $struct(std::u32::MAX)
            }

            fn from(value: u32) -> Self {
                $struct(value)
            }
        }
    };
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(super) struct TokenIndex(u32);
impl_index!(TokenIndex);

#[derive(Debug, Eq, Hash, PartialEq)]
pub(super) struct PathIndex(u32);
impl_index!(PathIndex);

#[derive(Debug, Eq, Hash, PartialEq)]
pub(super) struct FieldSetIndex(u32);
impl_index!(FieldSetIndex);

pub(super) struct StringIndex(u32);
impl_index!(StringIndex);

pub(super) struct FieldIndex(u32);
impl_index!(FieldIndex);

pub(super) enum IndexEnum {
    Token(u32),
    Path(u32),
    FieldSet(u32),
    String(u32),
    Field(u32),
}
