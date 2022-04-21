pub enum Primitive {
    Int {
      value: i64
    },
    Float {
      value: f64
    },
    Bool,
}

impl DataType for Primitive {

  fn stringify(&self) -> String {
    match self {
      Int:
    }
  }


}
