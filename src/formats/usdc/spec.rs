use super::index::{FieldSetIndex, PathIndex};

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
