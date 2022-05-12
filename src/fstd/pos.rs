use std::fmt::{Display, Formatter};

/// Struct to locate the position of point.
pub struct Position {
    line: usize,
    offset: usize,
}

impl Position {

    /// None position.
    pub const NONE: Self = Self {
        line: 0,
        offset: 0,
    };

    pub const START: Self = Self::new(1, 0);

    #[inline(always)]
    pub const fn new(line: usize, offset: usize) -> Self {
        assert!(line != 0, "");
        Self { line, offset }
    }

    #[inline(always)]
    pub const fn line(&self) -> usize {
        self.line
    }

    #[inline(always)]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    #[inline(always)]
    pub const fn is_none(&self) -> bool {
        self.line == 0 && self.offset == 0
    }
}

impl Default for Position {
    #[inline(always)]
    fn default() -> Self {
        Self::START
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_none() {
            write!(f, "none").unwrap();
        } else {
            write!(f, "line {}, position {}", self.line, self.offset).unwrap();
        }
        Ok(())
    }
}