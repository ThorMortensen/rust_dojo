// use crate::pkg_kit::pkg::Pkg;
// use rand::{thread_rng, Rng};

// #[test]
// fn test_empty_pkg() {
//     let pkg = Pkg::new();
//     assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "");
// }

// #[test]
// fn test_add_min_fields() {
//     let mut pkg = Pkg::new();
//     pkg.make_field("field0", 8, 1);
//     pkg.make_field("field1", 8, 1);
//     pkg.make_field("field2", 4, 1);
//     pkg.make_field("field3", 4, 1);
//     let out = pkg.to_bytes();
//     assert_eq!(hex::encode(out).to_ascii_uppercase(), "010111");
// }

// #[test]
// fn test_byte_size() {
//     let mut pkg = Pkg::new();
//     pkg.make_field("field0", 1, 0);
//     assert_eq!(pkg.get_field("field0").get_byte_size(), 1);
//     pkg.make_field("field1", 7, 0);
//     assert_eq!(pkg.get_field("field1").get_byte_size(), 1);
//     pkg.make_field("field2", 8, 0);
//     assert_eq!(pkg.get_field("field2").get_byte_size(), 1);
//     pkg.make_field("field3", 9, 0);
//     assert_eq!(pkg.get_field("field3").get_byte_size(), 2);
//     pkg.make_field("field4", 15, 0);
//     assert_eq!(pkg.get_field("field4").get_byte_size(), 2);
//     pkg.make_field("field5", 16, 0);
//     assert_eq!(pkg.get_field("field5").get_byte_size(), 2);
//     pkg.make_field("field6", 17, 0);
//     assert_eq!(pkg.get_field("field6").get_byte_size(), 3);
//     pkg.make_field("field7", 23, 0);
//     assert_eq!(pkg.get_field("field7").get_byte_size(), 3);
//     pkg.make_field("field8", 24, 0);
//     assert_eq!(pkg.get_field("field8").get_byte_size(), 3);
//     pkg.make_field("field9", 25, 0);
//     assert_eq!(pkg.get_field("field9").get_byte_size(), 4);
//     pkg.make_field("field10", 31, 0);
//     assert_eq!(pkg.get_field("field10").get_byte_size(), 4);
//     pkg.make_field("field11", 32, 0);
//     assert_eq!(pkg.get_field("field11").get_byte_size(), 4);
// }

// #[test]
// fn test_field_placement() {
//     let mut pkg = Pkg::new();
//     pkg.make_field("field0", 12, 0xB7B);
//     pkg.make_field("field1", 2, 1);
//     pkg.make_field("field2", 2, 3);
//     pkg.make_field("field3", 1, 0);
//     pkg.make_field("field4", 1, 0);
//     pkg.make_field("field5", 1, 1);
//     pkg.make_field("field6", 1, 0);
//     pkg.make_field("field7", 4, 0);
//     assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "B7B720");
// }

// fn gen_random_fields(pkg: &mut Pkg, field_count: u32) {
//     let mut rng = thread_rng();

//     for i in 0..field_count - 1 {
//         let size = rng.gen_range(1..=32);
//         let val = rng.gen_range(0..(1 << (size - 1)));
//         let name = format!("Field{}", i);
//         pkg.make_field(&name, size, val);
//         let field = pkg.get_field(&name);
//         assert_eq!(val, *field.get_value());
//         assert_eq!(size, *field.get_size());
//         assert_eq!(&pkg[i as usize], field);
//     }

//     if pkg.header_bits % 8 != 0 {
//         pkg.make_field(&format!("Field{}", field_count), (8 - (pkg.header_bits % 8)) as u32, 0);
//     }
// }

// #[test]
// fn test_add_random_fields() {
//     let mut pkg = Pkg::new();
//     gen_random_fields(&mut pkg, 127);
//     pkg.to_bytes();
//     // dbg!(pkg);
// }

// #[test]
// fn test_field_overflow() {
//     let mut pkg = Pkg::new();
//     pkg.make_field("field0", 2, 0);
//     pkg.make_field("field1", 6, 0x3BE);
//     assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "3E");
//     pkg[0].set_value(0xff);
//     assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "FE");
// }

// #[test]
// fn test_add_payload() {
//     let mut pkg = Pkg::new();
//     pkg.make_field("field", 8, 0xFF);
//     assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "FF");
//     let pl: Vec<u8> = vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];
//     pkg.add_payload(pl);
//     dbg!(&pkg);
//     assert_eq!(hex::encode(pkg.to_bytes()).to_ascii_uppercase(), "FF0102030405060708090A0B0C0D0E0F");
// }

// #[test]
// fn test_serialize_header() {
//     let mut pkg_out = Pkg::new();

//     pkg_out.make_field("field0", 1, 0);
//     pkg_out.make_field("field1", 7, 0);
//     pkg_out.make_field("field2", 16, 0);

//     let mut pkg_in = pkg_out.clone();

//     pkg_out.get_field_mut("field0").set_value(0x1);
//     pkg_out.get_field_mut("field1").set_value(0x18);
//     pkg_out.get_field_mut("field2").set_value(0x8001);

//     pkg_in.from_bytes(&pkg_out.to_bytes());

//     dbg!(&pkg_out);
//     dbg!(&pkg_in);
//     assert_eq!(hex::encode(pkg_in.to_bytes()).to_ascii_uppercase(), "988001");
// }

// #[test]
// fn test_serialize_with_payload() {
//     let mut pkg_out = Pkg::new();
//     let mut pkg_in = Pkg::new();

//     pkg_out.make_field("field0", 12, 0xB7B);
//     pkg_out.make_field("field1", 2, 1);
//     pkg_out.make_field("field2", 2, 3);
//     pkg_out.make_field("field3", 1, 0);
//     pkg_out.make_field("field4", 1, 0);
//     pkg_out.make_field("field5", 1, 1);
//     pkg_out.make_field("field6", 1, 0);
//     pkg_out.make_field("field7", 4, 9);

//     pkg_in.make_field("field0", 12, 0);
//     pkg_in.make_field("field1", 2, 0);
//     pkg_in.make_field("field2", 2, 0);
//     pkg_in.make_field("field3", 1, 0);
//     pkg_in.make_field("field4", 1, 0);
//     pkg_in.make_field("field5", 1, 0);
//     pkg_in.make_field("field6", 1, 0);
//     pkg_in.make_field("field7", 4, 0);


//     let pl: Vec<u8> = vec![0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF];
//     pkg_out.add_payload(pl);

//     let out = pkg_out.to_bytes();

//     pkg_in.from_bytes(&out);

//     dbg!(&pkg_out);
//     dbg!(&pkg_in);
//     assert_eq!(hex::encode(pkg_out.to_bytes()).to_ascii_uppercase(), "B7B7290102030405060708090A0B0C0D0E0F");
//     assert_eq!(hex::encode(pkg_in.to_bytes()).to_ascii_uppercase(), "B7B7290102030405060708090A0B0C0D0E0F");
// }
