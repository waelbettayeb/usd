use super::index::TokenIndex;
use super::value::ValueRep;

#[derive(Debug, Eq, Hash, PartialEq)]
pub(super) struct Field {
    unused: u32,
    token_index: TokenIndex,
    value_rep: ValueRep,
}
