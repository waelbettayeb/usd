use std::cmp::min;

use crate::formats::usdc::compress::decompress_from_buffer;
use nom::{number::complete::le_i32, IResult};
use nom_derive::{Nom, Parse};

const LZ4_MAX_INPUT_SIZE: usize = 0x7E000000;

#[derive(Nom, Debug)]
pub struct TokensSection {
    num_tokens: u64,
    uncompressed_size: u64,
    compressed_size: u64,
    #[nom(Parse = "decompress_from_buffer(uncompressed_size, compressed_size - 1)")]
    tokens: Vec<u8>,
}

fn parse_chunk(
    input: &[u8],
    compressed_size: usize,
    uncompressed_size: usize,
) -> IResult<&[u8], Vec<u8>> {
    let (input, rest) = input.split_at(compressed_size);
    let decompressed_bytes =
        lz4_flex::decompress(input, min(LZ4_MAX_INPUT_SIZE, uncompressed_size)).unwrap();
    // let value = std::str::from_utf8(&decompressed_bytes).unwrap();
    return Ok((rest, decompressed_bytes));
}

fn parse_chunk_size_prepended(
    uncompressed_size: usize,
) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<u8>> {
    move |input: &[u8]| {
        let (input, chunk_size) = le_i32(input)?;
        parse_chunk(input, chunk_size as usize, uncompressed_size)
    }
}

pub fn parse_tokens_section(input: &[u8]) -> IResult<&[u8], TokensSection> {
    TokensSection::parse_le(input)
}
