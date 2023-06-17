pub mod token;

use nom::{character::complete::anychar, multi::count, IResult};
use nom_derive::{Nom, Parse};

const SECTION_NAME_MAX_LENGTH: usize = 15;

#[derive(Nom, PartialEq, Eq, Hash, Debug)]
pub struct SectionName(#[nom(Parse = "parse_section_name")] [char; SECTION_NAME_MAX_LENGTH + 1]);

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

pub const STRINGS_SECTION_NAME: SectionName = SectionName::from_str("STRINGS");
pub const TOKENS_SECTION_NAME: SectionName = SectionName::from_str("TOKENS");
pub const FIELDSETS_SECTION_NAME: SectionName = SectionName::from_str("FIELDSETS");
pub const FIELDS_SECTION_NAME: SectionName = SectionName::from_str("FIELDS");
pub const PATHS_SECTION_NAME: SectionName = SectionName::from_str("PATHS");
pub const SPECS_SECTION_NAME: SectionName = SectionName::from_str("SPECS");

const KNOWN_SECTION_NAMES: [SectionName; 6] = [
    STRINGS_SECTION_NAME,
    TOKENS_SECTION_NAME,
    FIELDSETS_SECTION_NAME,
    FIELDS_SECTION_NAME,
    PATHS_SECTION_NAME,
    SPECS_SECTION_NAME,
];

#[derive(Nom, Debug)]
pub struct Section {
    pub name: SectionName,
    pub start: i64,
    pub size: i64,
}

#[derive(Nom)]
pub struct TableOfContent {
    pub sections: Vec<Section>,
}

fn parse_section_name(input: &[u8]) -> IResult<&[u8], [char; SECTION_NAME_MAX_LENGTH + 1]> {
    let (input, name_raw) = count(anychar, SECTION_NAME_MAX_LENGTH + 1)(input)?;
    let name: [char; SECTION_NAME_MAX_LENGTH + 1] = name_raw.try_into().unwrap();

    Ok((input, name))
}

pub fn parse_table(input: &[u8]) -> IResult<&[u8], TableOfContent> {
    TableOfContent::parse_le(input)
}

pub fn parse_fields(input: &[u8]) -> IResult<&[u8], ()> {
    Ok((input, ()))
}
#[derive(Nom, Debug)]
struct StringSection(Vec<u32>);
pub fn parse_strings(input: &[u8]) -> IResult<&[u8], Vec<u32>> {
    let (input, index) = StringSection::parse_le(input).unwrap();
    Ok((input, index.0))
}
