/**
 * Constant pool for those immutable variables.
 */
pub struct ConstantPool {
    pub integers: Vec<i64>,
}

const CONST_POOL_INIT_CAPACITY: u16 = 8;

impl ConstantPool {
    pub fn new() -> Self {
        ConstantPool {
            integers: Vec::with_capacity(8),
        }
    }
}
