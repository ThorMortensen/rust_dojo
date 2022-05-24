// pub mod tests;
pub mod pkg_kit;


#[cfg(test)]
mod tests {
  use super::pkg_kit::pkg::Pkg;
  use super::pkg_kit::field::Field;

  fn create_pkg() -> Pkg {
  
      let mut pkg = Pkg::new();
  
      pkg.add_field("field0", 4, 4);
      pkg.add_field("field1", 1, 1);
      pkg.add_field("field2", 1, 1);
      pkg.add_field("field3", 1, 1);
      pkg.add_field("field4", 1, 1);
  
      return pkg;
  }
  
      #[test]
    fn add_field() {
      let mut pkg = Pkg::new();
      pkg.add_field("field1", 1, 1);
      for i in 0..32{
        pkg.add_field(format!("field{}", i), 1, 1);
      }
      dbg!(pkg);
      for i in 0..32{
        let f = pkg.get_field(format!("field{}", i));
        assert_eq!(f.get_value(), 1);


      }
    }
}
