use nom::IResult;
use nom_derive::{NomLE, Parse};

use super::integers::decompress_integers;

#[derive(NomLE, Debug)]
pub struct PathsSection {
    number_of_paths: u64,
    num_paths: u64,

    indices_size: u64,
    #[nom(Parse = "decompress_integers(num_paths as usize, indices_size)")]
    indices: Vec<u32>,

    tokens_size: u64,
    #[nom(Parse = "decompress_integers(num_paths as usize, tokens_size)")]
    element_token_indices: Vec<i32>,

    jumbs_size: u64,
    #[nom(Parse = "decompress_integers(num_paths as usize, jumbs_size)")]
    jumbs: Vec<i32>,
}
pub fn parse_paths(input: &[u8]) -> IResult<&[u8], PathsSection> {
    PathsSection::parse(input)
}
