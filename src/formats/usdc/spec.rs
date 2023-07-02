use nom::IResult;
use nom_derive::{NomLE, Parse};

use super::index::{FieldSetIndex, PathIndex};
use super::integers::decompress_integers;

#[derive(Debug, Eq, Hash, PartialEq)]
enum SpecType {
    // The unknown type has a value of 0 so that SdfSpecType() is unknown.
    SdfSpecTypeUnknown = 0,

    // Real concrete types
    SdfSpecTypeAttribute,
    SdfSpecTypeConnection,
    SdfSpecTypeExpression,
    SdfSpecTypeMapper,
    SdfSpecTypeMapperArg,
    SdfSpecTypePrim,
    SdfSpecTypePseudoRoot,
    SdfSpecTypeRelationship,
    SdfSpecTypeRelationshipTarget,
    SdfSpecTypeVariant,
    SdfSpecTypeVariantSet,

    SdfNumSpecTypes,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub(super) struct Spec {
    path_index: PathIndex,
    field_set_index: FieldSetIndex,
    spec_type: SpecType,
}

#[derive(NomLE, Debug)]
pub struct SpecsSection {
    num_specs: u64,
    compressed_size: u64,
    #[nom(Parse = "decompress_integers(num_specs as usize, compressed_size)")]
    specs: Vec<u32>,
    #[nom(Count = "num_specs")]
    indices: Vec<u32>,
}
pub fn parse_specs_section(input: &[u8]) -> IResult<&[u8], SpecsSection> {
    SpecsSection::parse(input)
}
