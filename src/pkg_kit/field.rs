#[derive(Debug, Clone)]
pub struct Field {
    pub(super) bit_width: u32,
    pub(super) bit_mask: u32,
    pub(super) value: u32,
    pub(super) name: String,
}

// impl fmt::Debug for Field {
//     fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//         fmt.debug_struct("A")
//            .field("names", &self.names)
//            .finish()
//     }
// }

impl Field {
    pub(super) fn new(bit_width: u32, data: u32, name: String) -> Self {
        Self {
            bit_width: bit_width,
            bit_mask: 0xffffffff >> (32 - bit_width),
            value: data,
            name: name,
        }
    }

    pub fn set_data(&mut self, data: u32) {
        self.value = data & self.bit_mask;
    }

    pub(super) fn get_data(&self) -> &u32 {
        return &self.value;
    }

    // pub(super) fn to

    pub(super) fn get_bit_width(&self) -> &u32 {
        return &self.bit_width;
    }

    pub(super) fn get_byte_width(&self) -> u32 {
        return &self.bit_width / 8;
    }
}