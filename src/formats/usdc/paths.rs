use nom::IResult;
use nom_derive::{NomLE, Parse};

use super::integers::decompress_integers;
use crate::{path::Path, token::Token};

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
    let (input, section) = PathsSection::parse(input)?;
    let parent_path = Path::new();
    build_paths(&section, &parent_path);

    return Ok((input, section));
}

pub fn build_paths(section: &PathsSection, parent_path: &Path) {
    // TODO: add current index
    let mut has_child = false;
    let mut has_sibling = false;

    while has_child || has_sibling {
        let index = 0 + 1;
        has_child = section.jumbs[index] > 0 || section.jumbs[index] == -1;
        has_sibling = section.jumbs[index] >= 0;

        if parent_path.is_empty() {
            // TODO: add root absolute path
            Path::absolute_root();
            // TODO: get tokens
        } else {
            let token_index = section.element_token_indices[index];
            let is_prim_property_path = token_index < 0;
            let token_index = token_index.abs() as usize;
            // TODO: get tokens
            let token = Token::from("");
            // let element_token = section.token(token_index);
            // path[section.indices[current_index] as usize] = if is_prim_property_path {
            //     // append property
            parent_path.append_property(&token);
            // } else {
            //     // append element token
            parent_path.append_property(&token);
            // }
        }
    }
}
