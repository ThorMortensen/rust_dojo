use std::collections::HashMap;
extern crate hex;
// use std::rc;
// use std::convert;
// use std::ops::{Index, IndexMut};
use pad::{Alignment, PadStr};

// macro_rules! dbg {
//     ($x:expr) => {
//         println!("{} = {:?}",stringify!($x),$x);
//     }
// }

#[derive(Debug, Clone)]
pub struct Field {
    bit_width: u32,
    bit_mask: u32,
    data: u32,
    name: String,
}

impl Field {
    fn new(bit_width: u32, data: u32, name: String) -> Self {
        Self {
            bit_width: bit_width,
            bit_mask: 0xffffffff >> (32 - bit_width),
            data: data,
            name: name,
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

    pub(crate) fn get_byte_width(&self) -> u32 {
        return &self.bit_width / 8;
    }
}
const WORK_SPACE: usize = 5; // Insure extra workspace to avoid branching when compiling header fields

#[derive(Debug, Clone)]
pub struct Pkg {
    field_map: HashMap<String, usize>,
    field_vec: Vec<Field>,
    payload: Vec<u8>,
    header_bits: usize,
    pkg_compiled: Vec<u8>,
    is_compiled: bool,
}

// impl Index<usize> for Pkg {
//     type Output = Field;

//     fn index(&self, i: usize) -> Field {
//         &self.field_vec[i]
//     }
// }

// impl IndexMut<usize> for Pkg {
//     fn index_mut(&'a mut self, i: usize) -> &'a mut Field {
//         &mut self.field_vec[i]
//     }
// }

impl Pkg {
    pub fn new() -> Self {
        Self {
            field_map: HashMap::new(),
            field_vec: vec![],
            payload: vec![],
            header_bits: 0,
            pkg_compiled: vec![],
            is_compiled: false,
        }
    }

    pub fn add_field(&mut self, name: &str, bit_width: u32, data: u32) {
        self.header_bits += bit_width as usize;
        self.is_compiled = false;
        self.field_vec
            .push(Field::new(bit_width, data, name.to_owned()));
        self.field_map.insert(name.to_owned(), self.field_vec.len());
    }

    pub fn get_field_mut(&mut self, name: &str) -> Option<&mut Field> {
        if let Some(v) = self.field_map.get(name) {
            self.is_compiled = false;
            return self.field_vec.get_mut(*v);
        } else {
            return None;
        }
    }

    pub fn get_field(&mut self, name: &str) -> Option<&mut Field> {
        match self.field_map.get(name) {
            Some(v) => return self.field_vec.get_mut(*v),
            None => return None,
        }
    }

    pub fn add_payload(&mut self, payload: Vec<u8>) {
        self.is_compiled = false;
        self.payload = payload;
    }

    fn header_size(&self) -> usize {
        assert_eq!(
            self.header_bits % 8,
            0,
            "Header size must be devisable by 8"
        );
        return self.header_bits / 8;
    }

    fn size(&self) -> usize {
        return self.payload.len() + self.header_size();
    }

    pub fn to_bytes(&mut self) -> &[u8] {
        if !self.is_compiled {
            self.compile();
        }
        return &self.pkg_compiled[0..self.size()];
    }

    fn compile(&mut self) {
        let mut pkg = vec![0; self.header_size() + self.payload.len() + WORK_SPACE];
        let mut bits_used: u32 = 0;

        for field in &self.field_vec {
            let reminder: u32 = bits_used % 8;
            let shifted =
                (field.get_data().clone() as u64) << (64 - (reminder + field.get_bit_width()));

            let tail = bits_used as usize / 8;
            // Merge into header
            pkg[tail + 0] |= ((shifted >> 56) & 0xff) as u8; // Merge
            pkg[tail + 1] = ((shifted >> 48) & 0xff) as u8; // Inset
            pkg[tail + 2] = ((shifted >> 40) & 0xff) as u8; // Inset
            pkg[tail + 3] = ((shifted >> 32) & 0xff) as u8; // Inset
            pkg[tail + 4] = ((shifted >> 24) & 0xff) as u8; // Reminder (if any)
            bits_used += field.get_bit_width();
        }
        pkg[self.header_size()..self.header_size() + self.payload.len()]
            .copy_from_slice(&self.payload[..]);
        self.is_compiled = true;
        self.pkg_compiled = pkg;
    }

    fn print(&mut self) {
        println!("=========================");
        println!("Pkg size    : {} bytes", self.size());
        println!("Heder size  : {} bytes", self.header_size());
        println!("Payload size: {} bytes", self.payload.len());
        println!("-------- Fields ----------");
        println!("Name      : bits, value ");
        for field in &self.field_vec {
            println!(
                "{:10}: {:2},   0x{:0X} ",
                &field.name, &field.bit_width, &field.data
            )
        }
        println!("------------------------");
        println!("Bytes: {}", hex::encode(self.to_bytes()));
        println!("{}", "header | payload".pad(15 + (self.header_size() * 2),' ', Alignment::Right, false));
    }
    // pub fn from_bytes(&mut self, bytes: &Vec<u8>) {
    //     let mut asInt;

    //     // let mut pkg: Vec<u8> = vec![0; self.size() + WORK_SPACE];
    //     let mut idx: usize;
    //     let mut bits_used: u32 = 0;

    //     for field in &self.field_vec {}

    //     asInt = (asInt << 8) | p[0];
    //     asInt = (asInt << 8) | p[1];
    //     asInt = (asInt << 8) | p[2];
    //     asInt = (asInt << 8) | p[3];
    //     value = (asInt >> (32 - sizeBits)) & mask;
    // }

    // fn dismantle()
}

fn main() {
    let mut pkg = Pkg::new();
    pkg.add_field("fieldpre", 4, 4);
    pkg.add_field("field1", 1, 1);
    pkg.add_field("field2", 1, 1);
    pkg.add_field("field3", 1, 1);
    pkg.add_field("AAfield4", 1, 1);
    pkg.add_payload(vec![1, 2, 3, 4, 5]);

    pkg.print();

    let mut t: u64 = 0x123456789ABCDEF;

    // From<u64>

    // let data_out = pkg.to_bytes();
    // // dbg!(data_out);
}
