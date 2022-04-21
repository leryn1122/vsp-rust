pub struct VM {
    constant_pool: ConstantPool,
}

impl VM {
    pub fn new(constant_pool: ConstantPool) -> self {
        VM { constant_pool }
    }
}
