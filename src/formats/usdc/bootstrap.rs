use nom::bytes::complete::tag;
use nom::IResult;

use nom_derive::{Nom, Parse};

const BINARY_HEADER: &[u8; 8] = b"PXR-USDC";

#[derive(Nom)]
pub(super) struct Bootstrap {
    /// The binary header "PXR-USDC"
    #[nom(Parse = "parse_indent")]
    pub ident: [u8; 8],
    pub version: [u8; 8],
    pub toc_offset: i64,
    pub reserved: [i64; 8],
}

fn parse_indent(input: &[u8]) -> IResult<&[u8], [u8; 8]> {
    let (input, ident_array) = tag(BINARY_HEADER)(input)?;
    let ident: [u8; 8] = ident_array.try_into().unwrap();
    Ok((input, ident))
}

pub(super) fn parse_bootstrap(input: &[u8]) -> IResult<&[u8], Bootstrap> {
    Bootstrap::parse_le(input)
}
