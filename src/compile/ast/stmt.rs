use hashbrown::HashMap;
use crate::compile::ast::{
    Expression,
    StatementBlock,
};
use crate::SmartVec;

/// Conditional statement block.
///
/// ```txt
/// if condition -> statement
/// ```
pub struct ConditionStatementBlock {
    /// Condition expression return bool.
    pub condition: Expression,
    /// Statement if the condition return true.
    pub statements: StatementBlock,
}

///
///
///
pub struct SwitchStatementBlock {
    pub cases: HashMap<u64, Box<ConditionStatementBlock>>,
    pub default_case: Box<StatementBlock>,
    pub range: SmartVec<(i32, i32, bool, Box<StatementBlock>)>
}