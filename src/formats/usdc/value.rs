use std::fmt;

use super::value_type::ValueType;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(super) struct ValueRep(u64);

impl ValueRep {
    const IS_ARRAY_BIT: u64 = 1 << 63;
    const IS_INLINED_BIT: u64 = 1 << 62;
    const IS_COMPRESSED_BIT: u64 = 1 << 61;

    const PAYLOAD_MASK: u64 = (1 << 48) - 1;

    pub fn new() -> Self {
        ValueRep(0)
    }

    pub fn new_with_data(data: u64) -> Self {
        ValueRep(data)
    }

    pub fn new_with_type(t: ValueType, is_inlined: bool, is_array: bool, payload: u64) -> Self {
        ValueRep::combine(t, is_inlined, is_array, payload)
    }

    pub fn is_array(&self) -> bool {
        self.0 & ValueRep::IS_ARRAY_BIT != 0
    }

    pub fn set_is_array(&mut self) {
        self.0 |= ValueRep::IS_ARRAY_BIT;
    }

    pub fn is_inlined(&self) -> bool {
        self.0 & ValueRep::IS_INLINED_BIT != 0
    }

    pub fn set_is_inlined(&mut self) {
        self.0 |= ValueRep::IS_INLINED_BIT;
    }

    pub fn is_compressed(&self) -> bool {
        self.0 & ValueRep::IS_COMPRESSED_BIT != 0
    }

    pub fn set_is_compressed(&mut self) {
        self.0 |= ValueRep::IS_COMPRESSED_BIT;
    }

    pub fn get_type(&self) -> ValueType {
        let value_type = (self.0 >> 48) & 0xFF;
        ValueType::from(value_type)
    }

    pub fn set_type(&mut self, t: ValueType) {
        self.0 &= !(0xFF << 48);
        self.0 |= (t as u64) << 48;
    }

    pub fn get_payload(&self) -> u64 {
        self.0 & ValueRep::PAYLOAD_MASK
    }

    pub fn set_payload(&mut self, payload: u64) {
        self.0 &= !ValueRep::PAYLOAD_MASK;
        self.0 |= payload & ValueRep::PAYLOAD_MASK;
    }

    pub fn get_data(&self) -> u64 {
        self.0
    }

    fn combine(t: ValueType, is_inlined: bool, is_array: bool, payload: u64) -> Self {
        ValueRep(
            (if is_array { ValueRep::IS_ARRAY_BIT } else { 0 })
                | (if is_inlined {
                    ValueRep::IS_INLINED_BIT
                } else {
                    0
                })
                | ((t as u64) << 48)
                | (payload & ValueRep::PAYLOAD_MASK),
        )
    }
}

impl fmt::Display for ValueRep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValueRep({})", self.0)
    }
}
