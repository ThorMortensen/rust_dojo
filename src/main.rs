use std::collections::HashMap;

use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct Field {
    bit_width: u32,
    bit_mask: u32,
    data: u32,
}

impl Field {
    fn new(bit_width: u32, data: u32) -> Self {
        Self {
            bit_width: bit_width,
            bit_mask: 0xffffffff >> (32 - bit_width),
            data: data,
        }
    }

    pub fn setData(&mut self, data: u32) {
        self.data = data & self.bit_mask;
    }

    pub(crate) fn get_data(&self) -> &u32 {
        return &self.data;
    }

    pub(crate) fn get_bit_mask(&self) -> &u32 {
        return &self.bit_mask;
    }

    pub(crate) fn get_bit_width(&self) -> &u32 {
        return &self.bit_width;
    }
}

#[derive(Debug, Clone)]
pub struct Pkg {
    field_map: HashMap<String, usize>,
    field_vec: Vec<Field>,
    payload: Vec<u8>,
    header_size_bits: usize,
    pkg_compiled: Vec<u8>,
}

impl Index<usize> for Pkg {
    type Output = Field;

    fn index<'a>(&'a self, i: usize) -> &'a Field {
        &self.field_vec[i]
    }
}

impl IndexMut<usize> for Pkg {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Field {
        &mut self.field_vec[i]
    }
}


impl Pkg {
    pub fn new() -> Self {
        Self {
            field_map: HashMap::new(),
            field_vec: vec![],
            payload: vec![],
            header_size_bits: 0,
            pkg_compiled: vec![],
        }
    }

    pub fn add_field(&mut self, name: &str, bit_width: u32, data: u32) {
        self.header_size_bits += bit_width as usize;
        self.field_vec.push(Field::new(bit_width, data));
        self.field_map
            .insert(name.to_string(), self.field_vec.len());
    }

    pub fn get_field(&mut self, name: &str) -> Option<&mut Field> {
        match self.field_map.get(name) {
            Some(v) => return self.field_vec.get_mut(*v),
            None => return None,
        }
    }

    pub fn add_payload(&mut self, payload: Vec<u8>) {
        self.payload = payload;
    }

    pub fn compile(&mut self) -> &[u8] {
        assert_eq!(
            self.header_size_bits % 8,
            0,
            "Header size must be devisable by 8"
        );
        const WORK_SPACE: usize = 5; // Need some extra workspace to avoid branching when compiling header fields
        let pkg_size = self.payload.len() + (self.header_size_bits / 8);

        let mut pkg: Vec<u8> = vec![0; pkg_size + WORK_SPACE];
        let mut idx: usize;
        let mut bits_used: u32 = 0;
        // let mut reminder = 0;

        for field in &self.field_vec {
            idx = bits_used as usize / 8;
            let reminder: u32 = bits_used % 8;
            let shifted =
                (field.get_data().clone() as u64) << (64 - (reminder + field.get_bit_width()));
            bits_used += field.get_bit_width();

            pkg[idx + 0] |= ((shifted >> 56) & 0xff) as u8; // Merge
            pkg[idx + 1] = ((shifted >> 48) & 0xff) as u8; // Inset
            pkg[idx + 2] = ((shifted >> 40) & 0xff) as u8; // Inset
            pkg[idx + 3] = ((shifted >> 32) & 0xff) as u8; // Inset
            pkg[idx + 4] = ((shifted >> 24) & 0xff) as u8; // Reminder (if any)
        }
        self.pkg_compiled = pkg;

        return &self.pkg_compiled[0..pkg_size];
    }
}

fn main() {
    let mut pkg = Pkg::new();
    pkg.add_field("field1", 8, 1);
    pkg.add_field("field2", 8, 2);
    pkg.add_field("field3", 8, 3);
    pkg.add_field("AAfield4", 8, 4);

    let data_out = pkg.compile();

    println!("data_out {:?}", data_out);
}
