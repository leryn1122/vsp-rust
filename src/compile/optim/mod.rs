
/// Level of optimization.
#[derive(Debug, PartialEq, Hash)]
pub enum OptimizationLevel {
    /// Never optimize the code.
    None,
    /// Full optimization.
    Full,
}

impl Default for OptimizationLevel {
    #[inline(always)]
    fn default() -> Self {
        Self::None
    }
}