use std::fmt;

#[derive(Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub struct Field {
    pub(super) size: u32,
    pub(super) bit_mask: u32,
    pub(super) value: u32,
    pub(super) name: String,
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:<14}: {:>5}, 0x{:0X}\n", self.name, self.size, self.value)
    }
}

#[allow(dead_code)]
impl Field {
    pub(super) fn new(size: u32, value: u32, name: String) -> Self {
        assert!(size <= 32, "Size is {}. Size of fields must be less that 32 bits", size);
        let bit_mask =  0xffffffff >> (32 - size);
        Self {
            size,
            bit_mask: bit_mask,
            value: value & bit_mask,
            name,
        }
    }

    pub fn set_value(&mut self, data: u32) {
        self.value = data & self.bit_mask;
    }

    pub fn get_value(&self) -> &u32 {
        return &self.value;
    }

    pub(super) fn get_size(&self) -> &u32 {
        return &self.size;
    }

    pub(super) fn get_byte_size(&self) -> u32 {
        return &self.size / 8;
    }
}