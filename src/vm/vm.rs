use crate::vm::constant::ConstantPool;
use crate::vm::static_table::StaticTable;

pub struct VirtualMachine {
    constant_pool: ConstantPool,
    static_table: StaticTable,
}

impl VirtualMachine {

    pub fn new() -> Self {
        VirtualMachine {
            constant_pool: ConstantPool::new(),
            static_table: StaticTable::new()
        }
    }

    /// Returns the PID.
    pub fn start(&self) -> usize {
        0
    }
}
