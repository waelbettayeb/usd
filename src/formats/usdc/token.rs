use crate::formats::usdc::compress::decompress_from_buffer;
use nom::IResult;
use nom_derive::{NomLE, Parse};

#[derive(NomLE, Debug)]
pub struct TokensSection {
    num_tokens: u64,
    uncompressed_size: u64,
    compressed_size: u64,
    #[nom(Parse = "decompress_from_buffer(uncompressed_size, compressed_size)")]
    tokens: Vec<u8>,
}

pub fn parse_tokens_section(input: &[u8]) -> IResult<&[u8], TokensSection> {
    TokensSection::parse(input)
}
