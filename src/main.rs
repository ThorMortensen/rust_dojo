use rust_dojo::pkg_kit::pkg::Pkg;


fn create_pkg() -> Pkg {

    let mut pkg = Pkg::new();


    pkg.add_field("field0", 4, 4);
    pkg.add_field("field1", 1, 1);
    pkg.add_field("field2", 12, 1);
    pkg.add_field("field3", 1, 1);
    pkg.add_field("field4", 1, 1);

    return pkg;
}


fn main() {
    let mut pkg = create_pkg();
    let mut pkg2 = create_pkg();

    pkg.add_payload(vec![1, 2, 3, 4, 5]);

    pkg.get_field_mut("field2").set_data(2);

    let p = pkg.to_bytes();

    pkg.print();
    pkg2.print();


    pkg.from_bytes(&p);
    pkg.print();

}
