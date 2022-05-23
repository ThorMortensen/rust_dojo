extern crate hex;
use core::fmt;
use pad::{Alignment, PadStr};
use std::collections::HashMap;

use super::field::Field;

#[derive(Debug, Clone)]
pub struct Pkg {
    field_map: HashMap<String, usize>,
    field_vec: Vec<Field>,
    payload: Vec<u8>,
    header_bits: usize,
    is_compiled: bool,
}

impl Pkg {
    pub fn new() -> Self {
        Self {
            field_map: HashMap::new(),
            field_vec: vec![],
            payload: vec![],
            header_bits: 0,
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

    pub fn get_field_mut(&mut self, name: &str) -> &mut Field {
        self.is_compiled = false;
        let err_msg = |n| format!("Field '{}' doesn't exist!", n);
        let idx = *self.field_map.get(name).expect(err_msg(name).as_str());
        return self.field_vec.get_mut(idx).expect(err_msg(name).as_str());
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

    pub fn to_bytes(&mut self) -> Vec<u8> {
        let mut pkg = vec![0; self.header_size() + self.payload.len()];
        let mut bits_used: u32 = 0;

        for field in &self.field_vec {
            let reminder: u32 = bits_used % 8;
            let shifted_bytes = ((field.get_data().clone() as u64)
                << (64 - (reminder + field.get_bit_width())))
            .to_be_bytes();
            let tail = (bits_used / 8) as usize;
            let field_byte_count = (field.get_bit_width() / 8) as usize;
            // dbg!(shifted_bytes);
            // dbg!(tail);
            // dbg!(field_byte_count);

            bits_used += field.get_bit_width();

            for i in 0..=field_byte_count {
                pkg[i + tail] |= shifted_bytes[i];
            }
        }
        pkg[self.header_size()..self.header_size() + self.payload.len()]
            .copy_from_slice(&self.payload[..]);
        return pkg;
    }

    pub fn from_bytes(&mut self, bytes: &[u8]) {
        let mut bits_used: u32 = 0;

        for field in &mut self.field_vec {
            let tail = bits_used as usize / 8;
            bits_used += field.get_bit_width();
            let mut as_int: u32 = 0;
            let field_byte_count = (field.get_bit_width() / 8) as usize;

            for i in tail..=tail + field_byte_count {
                as_int = (as_int << 8) | bytes[i] as u32;
            }

            field.value = (as_int >> field.bit_width) & field.bit_mask;
        }
    }

    pub fn print(&mut self) {
        println!("=========================");
        println!("Pkg size    : {} bytes", self.size());
        println!("Heder size  : {} bytes", self.header_size());
        println!("Payload size: {} bytes", self.payload.len());
        println!("-------- Fields ---------");
        println!("Name      : bits, value ");
        for field in &self.field_vec {
            println!(
                "{:10}: {:2},   0x{:0X} ",
                &field.name, &field.bit_width, &field.value
            )
        }
        println!("------------------------");
        println!("Bytes: {}", hex::encode(self.to_bytes()));
        println!(
            "{}",
            "header | payload".pad(15 + (self.header_size() * 2), ' ', Alignment::Right, false)
        );
    }
}


impl fmt::Debug for BlahLF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hi: {}", self.id)
    }
}