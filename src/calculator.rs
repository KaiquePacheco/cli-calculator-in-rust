pub mod Term {
  pub fn new_number(value: f32) -> TermNode{
    TermNode::number{value: value}
  }
  pub enum TermNode {
    number{value: f32}
  }

  impl TermNode {
    pub fn get_value(&self) -> f32{
      match get_value_result() {
        Ok(value) => return value,
        Err(log) => panic!(log)
      }
    }
    pub fn get_value_result(&self) -> Result<f32, &str>{
      if let TermNode::number{value} = self{
        return Ok(value);
      }
      return Err("This term node is not a number, but yes a operation.");
    }
  }
}