
use std::fmt;

#[derive(Clone)]
#[allow(dead_code)]
pub struct Field {
    pub(super) bit_width: u32,
    pub(super) bit_mask: u32,
    pub(super) value: u32,
    pub(super) name: String,
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:10}: {:2},   0x{:0X}\n", self.name, self.bit_width, self.value)
    }
}

impl Field {
    pub(super) fn new(bit_width: u32, data: u32, name: String) -> Self {
        Self {
            bit_width: bit_width,
            bit_mask: 0xffffffff >> (32 - bit_width),
            value: data,
            name: name,
        }
    }

    pub fn set_value(&mut self, data: u32) {
        self.value = data & self.bit_mask;
    }

    pub fn get_value(&self) -> &u32 {
        return &self.value;
    }

    pub(super) fn get_bit_width(&self) -> &u32 {
        return &self.bit_width;
    }

    pub(super) fn get_byte_width(&self) -> u32 {
        return &self.bit_width / 8;
    }
}