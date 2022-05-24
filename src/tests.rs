


#[cfg(test)]
mod tests {
  use super::*;

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
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
