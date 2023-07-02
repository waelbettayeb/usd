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

    use crate::formats::usdc::field::{parse_field_sets_section, parse_fields_section};
    use crate::formats::usdc::paths::parse_paths;
    use crate::formats::usdc::spec::parse_specs_section;

    use crate::formats::usdc::bootstrap::parse_bootstrap;
    use crate::formats::usdc::sections::{
        parse_strings, parse_table, FIELDSETS_SECTION_NAME, FIELDS_SECTION_NAME,
        PATHS_SECTION_NAME, SPECS_SECTION_NAME, STRINGS_SECTION_NAME, TOKENS_SECTION_NAME,
    };
    use crate::formats::usdc::token::parse_tokens_section;

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
                    let (_, section) = parse_strings(input).unwrap();
                    println!("STRINGS: {:?}", section);
                }
                TOKENS_SECTION_NAME => {
                    let (_, section) = parse_tokens_section(input).unwrap();
                    println!("TOKENS: {:?}", section);
                }
                FIELDSETS_SECTION_NAME => {
                    let (_, section) = parse_field_sets_section(input).unwrap();
                    println!("FIELDSETS: {:?}", section);
                }
                FIELDS_SECTION_NAME => {
                    let (_, section) = parse_fields_section(input).unwrap();
                    println!("FIELDS: {:?}", section);
                }
                PATHS_SECTION_NAME => {
                    let (_, section) = parse_paths(input).unwrap();
                    println!("PATHS: {:?}", section);
                }
                SPECS_SECTION_NAME => {
                    let (_, section) = parse_specs_section(input).unwrap();
                    println!("SPECS: {:?}", section);
                }
                _ => {
                    println!("No match: {:?}", section.name)
                }
            }
        });
        assert_ne!(contents.len(), 882);
    }
}
