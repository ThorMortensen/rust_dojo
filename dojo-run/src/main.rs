use pkg_kit::pkg::Pkg;

fn main() {
    let mut pkg = Pkg::new();
    pkg.make_field("field0", 8, 1);
    pkg.make_field("field1", 8, 1);
    pkg.make_field("field2", 4, 1);
    pkg.make_field("field3", 4, 1);
    // let out = pkg.to_bytes();
    dbg!(pkg);
}

// #[derive(Debug, Copy, Clone)]
// struct Bar {
//     val: u32,
// }
// #[derive(Debug, Copy, Clone)]
// enum Foo {
//     A,
//     B,
//     D,
//     E(Bar),
// }

// fn main() {
//     let f = Foo::E(Bar { val: 42 });

//     for _i in 0..10 {
//         match f {
//             Foo::A => todo!(),
//             Foo::B => todo!(),
//             Foo::D => todo!(),
//             Foo::E(mut inner) => {
//                 inner.val += 1;
//                 Foo::E(inner);
//             }
//         }
//         dbg!(&f);
//     }
// }
