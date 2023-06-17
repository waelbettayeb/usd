//! A usdc file is structured as follows:
//!
//! * A bootstrap block
//! * A table of contents
//! * A table of contents range
//! * A tokens section
//! * A strings section
//! * A fields section
//! * A field sets section
//! * A paths section
//! * A specs section
//!
//!

#[cfg(test)]
mod usdc {

    use super::super::bootstrap::parse_bootstrap;
    use super::super::sections::token::parse_tokens_section;
    use super::super::sections::{
        parse_fields, parse_strings, parse_table, FIELDSETS_SECTION_NAME, FIELDS_SECTION_NAME,
        PATHS_SECTION_NAME, SPECS_SECTION_NAME, STRINGS_SECTION_NAME, TOKENS_SECTION_NAME,
    };

    use nom::Slice;

    #[test]
    fn test_parse_usdc() {
        let contents: &[u8] = include_bytes!("../../assets/crate.usdc");
        let (_, bootstrap) = parse_bootstrap(contents).unwrap();

        // get toc from the end of the file
        let offset = bootstrap.toc_offset as usize + 8usize;
        let toc_input = contents.slice(offset..);
        let (_, toc) = parse_table(toc_input).unwrap();

        toc.sections.iter().for_each(|section| {
            let input =
                contents.slice((section.start) as usize..(section.start + section.size) as usize);
            match section.name {
                STRINGS_SECTION_NAME => {
                    parse_strings(input).unwrap();
                }
                TOKENS_SECTION_NAME => {
                    let (_, token_section) = parse_tokens_section(input).unwrap();
                    println!("token_section: {:?}", token_section);
                }
                FIELDSETS_SECTION_NAME => {
                    println!("FIELDSETS: {:?}", section);
                }
                FIELDS_SECTION_NAME => {
                    parse_fields(input).unwrap();
                }
                PATHS_SECTION_NAME => {
                    println!("PATHS: {:?}", section);
                }
                SPECS_SECTION_NAME => {
                    println!("SPECS: {:?}", section);
                }
                _ => {
                    println!("No match: {:?}", section.name)
                }
            }
        });
        assert_eq!(contents.len(), 882);
    }
}
