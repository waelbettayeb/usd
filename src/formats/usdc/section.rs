use std::{error::Error, io::Read, str::FromStr};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::anychar,
    combinator::map_res,
    multi::{count, many0},
    sequence::terminated,
    IResult,
};
use nom_derive::{Nom, Parse};

const SECTION_NAME_MAX_LENGTH: usize = 15;

#[derive(Nom, PartialEq, Eq, Hash, Debug)]
pub(super) struct SectionName(
    #[nom(Parse = "parse_section_name")] [char; SECTION_NAME_MAX_LENGTH + 1],
);

impl SectionName {
    const fn from_str(input: &str) -> SectionName {
        let bytes = input.as_bytes();

        let mut array: [char; 16] = ['\0'; 16];
        let mut index = 0;

        while index < 16 && index < bytes.len() {
            array[index] = bytes[index] as char;
            index += 1;
        }

        SectionName(array)
    }
}
// impl PartialEq for SectionName {
//     fn eq(&self, other: &Self) -> bool {
//         let name = self.0;
//         let other_name = other.0;

//         let mut index = 0;
//         while index < 16 && name[index] != '\0' && other_name[index] != '\0' {
//             if name[index] != other_name[index] {
//                 return false;
//             }
//             index += 1;
//         }
//         true
//     }
// }

pub(super) const STRINGS_SECTION_NAME: SectionName = SectionName::from_str("STRINGS");
pub(super) const TOKENS_SECTION_NAME: SectionName = SectionName::from_str("TOKENS");
pub(super) const FIELDSETS_SECTION_NAME: SectionName = SectionName::from_str("FIELDSETS");
pub(super) const FIELDS_SECTION_NAME: SectionName = SectionName::from_str("FIELDS");
pub(super) const PATHS_SECTION_NAME: SectionName = SectionName::from_str("PATHS");
pub(super) const SPECS_SECTION_NAME: SectionName = SectionName::from_str("SPECS");

const KNOWN_SECTION_NAMES: [SectionName; 6] = [
    STRINGS_SECTION_NAME,
    TOKENS_SECTION_NAME,
    FIELDSETS_SECTION_NAME,
    FIELDS_SECTION_NAME,
    PATHS_SECTION_NAME,
    SPECS_SECTION_NAME,
];

#[derive(Nom, Debug)]
pub(super) struct Section {
    pub name: SectionName,
    pub start: i64,
    pub size: i64,
}

#[derive(Nom)]
pub(super) struct TableOfContent {
    pub sections: Vec<Section>,
}

fn parse_section_name(input: &[u8]) -> IResult<&[u8], [char; SECTION_NAME_MAX_LENGTH + 1]> {
    let (input, name_raw) = count(anychar, SECTION_NAME_MAX_LENGTH + 1)(input)?;
    let name: [char; SECTION_NAME_MAX_LENGTH + 1] = name_raw.try_into().unwrap();

    Ok((input, name))
}

pub(super) fn parse_table(input: &[u8]) -> IResult<&[u8], TableOfContent> {
    TableOfContent::parse_le(input)
}

#[derive(Nom, Debug)]
pub(super) struct TokensSection {
    num_tokens: u64,
    uncompressed_size: u64,
    compressed_size: u64,
    #[nom(Parse = "parse_tokens")]
    tokens: Vec<String>,
}

pub(super) fn parse_fields(input: &[u8]) -> IResult<&[u8], ()> {
    Ok((input, ()))
}
pub(super) fn parse_strings(input: &[u8]) -> IResult<&str, ()> {
    let result = std::str::from_utf8(input).unwrap();
    Ok((result, ()))
}

pub(super) fn parse_tokens(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    let string_parser = take_until("\0");
    let contiguous_string_parser = terminated(string_parser, tag("\0"));

    let parser = map_res(contiguous_string_parser, |bytes: &[u8]| {
        std::str::from_utf8(bytes).map(|s| s.to_string())
    });
    many0(parser)(input)
}

pub(super) fn parse_tokens_section(input: &[u8]) -> IResult<&[u8], TokensSection> {
    TokensSection::parse_le(input)
}
