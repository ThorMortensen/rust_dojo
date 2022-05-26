use crate::pkg_kit::pkg::Pkg;
use rand::{thread_rng, Rng};


#[test]
pub fn test_empty_pkg() {
    let pkg = Pkg::new();
    assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "");
}

#[test]
pub fn test_add_min_fields() {
    let mut pkg = Pkg::new();
    pkg.make_field("field0", 8, 1);
    assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "01");
}

#[test]
pub fn test_field_placement() {
    let mut pkg = Pkg::new();
    pkg.make_field("field0", 12, 0xB7B);
    pkg.make_field("field000", 2, 1);
    pkg.make_field("field00", 2, 3);
    pkg.make_field("field1", 1, 0);
    pkg.make_field("field2", 1, 0);
    pkg.make_field("field3", 1, 1);
    pkg.make_field("field4", 1, 0);
    pkg.make_field("field5", 4, 0);
    assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "B7B720");
}

#[test]
pub fn test_add_random_fields() {
    let mut pkg = Pkg::new();
    let mut rng = thread_rng();

    for i in 0..127 {
        let size = rng.gen_range(1..=32);
        let val = rng.gen_range(0..(1 << (size - 1)));
        let name = format!("Field{}", i);
        pkg.make_field(&name, size, val);
        let field = pkg.get_field(&name);
        assert_eq!(val, *field.get_value());
        assert_eq!(size, *field.get_size());
        assert_eq!(&pkg[i], field);
    }

    if pkg.header_bits % 8 != 0 {
        pkg.make_field("Field128", (8 - (pkg.header_bits % 8)) as u32, 0);
    }
    pkg.to_bytes();
    // dbg!(pkg);
}

#[test]
pub fn test_field_overflow() {
    let mut pkg = Pkg::new();
    pkg.make_field("field0", 2, 0);
    pkg.make_field("field1", 6, 0x3BE);
    assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "3E");
    pkg[0].set_value(0xff);
    assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "FE");
}

#[test]
pub fn test_add_payload() {
    let mut pkg = Pkg::new();
    pkg.make_field("field", 8, 0xFF);
    assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "FF");
    // let pl: Vec<u8> = vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];
    // pkg.add_payload(pl);
    // dbg!(&pkg);
    // assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "FF123456789ABCDEF");
}

#[test]
pub fn test_compile_decompile() {
    // let mut pkg = Pkg::new();
    // pkg.make_field("field", 8, 0xFF);
    // assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "FF");
    // let pl: Vec<u8> = vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];
    // pkg.add_payload(pl);
    // dbg!(&pkg);
    // assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "FF123456789ABCDEF");
}