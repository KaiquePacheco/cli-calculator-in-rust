pub mod term {
  use super::operation::Operation;

  pub fn new_number(value: f32) -> TermNode{
    TermNode::Number{value: value}
  }

  pub enum TermNode {
    Number{value: f32},
    Operation{operation: Operation}
  }

  impl TermNode {
    pub fn get_value(&self) -> f32{
      self.get_value_result().unwrap()
    }
    pub fn get_value_result(&self) -> Result<f32, &str>{
      if let TermNode::Number{value} = self{
        return Ok(*value)
      }
      return Err("This term node is not a number, but yes a operation.")
    }

    pub fn operate(&self) -> TermNode{
      self.operate_result().unwrap()
    }
    pub fn operate_result(&self) -> Result<TermNode, &str>{
      if let TermNode::Operation { operation } = self{
        return Ok(operation.operate())
      }
      return Err("This term node is not a operation, but yes a number")
    }
  }
}
pub mod operation {
  use super::term;
  use term::TermNode;

  static OPERATION_SYMBOLS: [&str; 2] = ["+", "-"];

  pub fn new_addition(addend_1: TermNode, addend_2: TermNode) -> TermNode{
    TermNode::Operation { 
      operation: Operation::Addition { 
        addend_1: Box::new(addend_1),
        addend_2: Box::new(addend_2) 
      }
    }
  }
  pub fn new_subtract(minuend: TermNode, subtrahend: TermNode) -> TermNode{
    TermNode::Operation { 
      operation: Operation::Subtract { 
        minuend: Box::new(minuend),
        subtrahend: Box::new(subtrahend)
      }
    }
  }

  pub enum Operation{
    Addition{addend_1: Box<TermNode>, addend_2: Box<TermNode>},
    Subtract{minuend: Box<TermNode>, subtrahend: Box<TermNode>}
  }
  impl Operation{
    pub fn operate(&self) -> TermNode{
      match self {
        Operation::Addition { addend_1, addend_2 } => {
          let addend_1 = addend_1.get_value();
          let addend_2 = addend_2.get_value();

          return term::new_number(addend_1 + addend_2)
        } 
        Operation::Subtract { minuend, subtrahend } => {
          let minuend = minuend.get_value();
          let subtrahend = subtrahend.get_value();

          return term::new_number(minuend - subtrahend)
        }
      }
    }
  }
}
#[cfg(test)]
mod test_term{
  use test_case::test_case;

  use super::term;
  use super::operation;

  #[test_case(3.4; "Get the number 3.4 from a number term node.")]
  #[test_case(22.3; "Get the number 22.3 from a number term node.")]
  #[test_case(-34.0; "Get the number -34.0 from a number term node.")]
  fn number_term_get(v: f32){
    let number_value = term::new_number(v).get_value();
    assert_eq!(number_value, v);
  }

  #[test_case(3.2, 2.3; "Sum 3.2 and 2.3.")]
  #[test_case(0.0, 2.3; "Sum 0 and 2.3.")]
  #[test_case(3.2, -1.0; "Sum 3.2 and -1.0.")]
  fn operate_addition(v1: f32, v2: f32){
    let addend_1 = term::new_number(v1);
    let addend_2 = term::new_number(v2);

    let value = operation::new_addition(addend_1, addend_2)
    .operate()
    .get_value();

    assert_eq!(value, v1 + v2);
  }

  #[test_case(1.9, 3.1; "Subtract 1.9 to 3.1.")]
  #[test_case(0.0, 3.1; "Subtract 0 to 3.1.")]
  #[test_case(100.0, 90.1; "Subtract 100.0 to 90.1.")]
  fn operate_subtract(v1: f32, v2: f32){
    let minuend = term::new_number(v1);
    let subtrahend = term::new_number(v2);

    let value = operation::new_subtract(minuend, subtrahend)
    .operate()
    .get_value();

    assert_eq!(value, v1 - v2);
  }
}