extern crate hex;

use core::fmt;
use pad::{Alignment, PadStr};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use super::field::Field;
#[cfg(test)]
#[path = "tests.rs"]
pub mod tests;
#[derive(Clone)]
#[allow(dead_code)]
pub struct Pkg {
    field_map: HashMap<String, usize>,
    field_vec: Vec<Field>,
    payload: Vec<u8>,
    header_bits: usize,
    is_compiled: bool,
}

macro_rules! err_msg_no_field {
    ($name:expr) => {
        &format!("Field {} doesn't exist!", $name).to_owned()
    };
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

    pub fn make_field(&mut self, name: &str, size: u32, value: u32) {
        assert!(!self.field_map.contains_key(name), "A field named '{}' already exists", name);
        self.header_bits += size as usize;
        self.is_compiled = false;
        self.field_vec.push(Field::new(size, value, name.to_owned()));
        self.field_map.insert(name.to_owned(), self.field_vec.len() - 1);
    }

    pub fn get_field_mut(&mut self, name: &str) -> &mut Field {
        self.is_compiled = false;
        let idx = *self.field_map.get(name).expect(err_msg_no_field!(name));
        return &mut self[idx];
    }

    pub fn get_field(&self, name: &str) -> &Field {
        let idx = *self.field_map.get(name).expect(err_msg_no_field!(name));
        return &self[idx];
    }

    pub fn add_payload(&mut self, payload: Vec<u8>) {
        self.is_compiled = false;
        self.payload = payload;
    }

    fn header_size(&self) -> usize {
        assert_eq!(self.header_bits % 8, 0, "Header size must be devisable by 8");
        return self.header_bits / 8;
    }

    fn size(&self) -> usize {
        return self.payload.len() + self.header_size();
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut pkg = vec![0; self.header_size() + self.payload.len()];
        let mut bits_used: u32 = 0;

        for field in &self.field_vec {
            let reminder: u32 = bits_used % 8;
            let shifted_bytes = ((field.get_value().clone() as u64) << (64 - (reminder + field.get_size()))).to_be_bytes();
            let tail = (bits_used / 8) as usize;
            let field_byte_count = (field.get_size() / 8) as usize;
            bits_used += field.get_size();

            for i in 0..field_byte_count {
                pkg[i + tail] |= shifted_bytes[i];
            }
        }
        pkg[self.header_size()..self.header_size() + self.payload.len()].copy_from_slice(&self.payload[..]);
        return pkg;
    }

    pub fn from_bytes(&mut self, bytes: &[u8]) {
        let mut bits_used: u32 = 0;

        for field in &mut self.field_vec {
            let tail = bits_used as usize / 8;
            bits_used += field.get_size();
            let mut as_int: u32 = 0;
            let field_byte_count = (field.get_size() / 8) as usize;

            for i in tail..=tail + field_byte_count {
                as_int = (as_int << 8) | bytes[i] as u32;
            }

            field.value = (as_int >> field.size) & field.bit_mask;
        }
    }
}

impl Index<usize> for Pkg {
    type Output = Field;
    fn index (& self, i: usize) -> &Field {
        return &self.field_vec.get(i).expect(err_msg_no_field!(&i.to_string()));
    }
}

impl IndexMut<usize> for  Pkg {
    fn index_mut(&mut self, i: usize) -> &mut Field {
        return self.field_vec.get_mut(i).expect(err_msg_no_field!(&i.to_string()));
    }
}

impl fmt::Debug for Pkg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n=========================\n")?;
        write!(f, "Pkg size    : {:>4} bytes\n", self.size())?;
        write!(f, "Heder size  : {:>4} bytes\n", self.header_size())?;
        write!(f, "Payload size: {:>4} bytes\n", self.payload.len())?;
        write!(f, "-------- Fields ---------\n")?;
        write!(f, "Name          : bits, value \n")?;
        for field in &self.field_vec {
            write!(f, "{:?}", field)?;
        }
        write!(f, "------------------------\n")?;
        write!(f, "Bytes: {}\n", hex::encode(self.to_bytes()))?;
        write!(f, "{}", "header | payload\n".pad(15 + (self.header_size() * 2), ' ', Alignment::Right, false))
    }
}
