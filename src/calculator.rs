pub mod term {
  pub fn new_number(value: f32) -> TermNode{
    TermNode::number{value: value}
  }
  pub enum TermNode {
    number{value: f32}
  }

  impl TermNode {
    pub fn get_value(&self) -> f32{
      match self.get_value_result() {
        Ok(value) => return value,
        Err(log) => panic!("{log}")
      }
    }
    pub fn get_value_result(&self) -> Result<f32, &str>{
      if let TermNode::number{value} = self{
        return Ok(*value);
      }
      return Err("This term node is not a number, but yes a operation.");
    }
  }
}
#[cfg(test)]
mod test_term{
  use test_case::test_case;
  use super::term;

  #[test_case(3.4; "Get the number 3.4 from a number term node.")]
  #[test_case(22.3; "Get the number 22.3 from a number term node.")]
  #[test_case(-34.0; "Get the number -34.0 from a number term node.")]
  fn number_term_get(v: f32){
    let number_value = term::new_number(v).get_value();
    assert_eq!(number_value, v);
  }

}