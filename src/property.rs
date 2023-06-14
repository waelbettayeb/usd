use crate::{attribute::Attribute, relationship::Relationship};

pub enum Property {
    Relationship(Relationship),
    Attribute(Attribute),
}
