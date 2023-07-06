use crate::{formats::usdc::compress::decompress_from_buffer, token::Token};
use nom::IResult;
use nom_derive::{NomLE, Parse};

#[derive(NomLE, Debug)]
pub struct TokensSection {
    num_tokens: u64,
    uncompressed_size: u64,
    compressed_size: u64,
    #[nom(Parse = "decompress_from_buffer(uncompressed_size, compressed_size)")]
    buffer: Vec<u8>,
}

pub fn parse_tokens_section(input: &[u8]) -> IResult<&[u8], Vec<Token>> {
    let (input, section) = TokensSection::parse(input)?;
    let tokens: Vec<Token> = section
        .buffer
        .split(|&b| b == b'\0')
        .map(|bytes| Token::from(String::from_utf8_lossy(bytes).into_owned()))
        .collect();

    return Ok((input, tokens));
}
