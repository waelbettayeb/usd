use nom::IResult;
use nom_derive::{NomLE, Parse};

use super::index::TokenIndex;
use super::integers::decompress_integers;
use super::value::ValueRep;
#[derive(Debug, Eq, Hash, PartialEq)]
pub(super) struct Field {
    unused: u32,
    token_index: TokenIndex,
    value_rep: ValueRep,
}

#[derive(NomLE, Debug)]
pub struct FieldSection {
    num_fields: u64,
    compressed_size: u64,
    #[nom(Parse = "decompress_integers(num_fields as usize, compressed_size)")]
    indices: Vec<u32>,
    representations_size: u64,
    #[nom(Parse = "decompress_integers(num_fields as usize, representations_size)")]
    representations: Vec<u64>,
}

pub fn parse_fields_section(input: &[u8]) -> IResult<&[u8], FieldSection> {
    FieldSection::parse(input)
}

#[derive(NomLE, Debug)]
pub struct PathsSection {
    num_paths: u64,
    #[nom(Count = "num_paths")]
    indices: Vec<u32>,
    #[nom(Count = "num_paths")]
    element_token_indices: Vec<i32>,
    #[nom(Count = "num_paths")]
    jumbs: Vec<i32>,
}
pub fn parse_paths(input: &[u8]) -> IResult<&[u8], PathsSection> {
    PathsSection::parse(input)
}
