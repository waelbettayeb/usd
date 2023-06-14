use crate::{property::Property, schema::Schema};

struct Layer<'a> {
    pub schema: &'a Schema,
    properties: Vec<Property>,
}
