pub fn eprint_error(input: &str) {
    todo!("{}", input)
}

#[allow(unused_variables, dead_code)]
fn eprint_line(lines: &[&str], pos: Position, err_msg: &str) {
    eprintln!();
    eprintln!("");
}


pub struct Position {
    line: usize,
    offset: usize,
}

impl Position {

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
}

impl Default for Position {
    #[inline(always)]
    fn default() -> Self {
        Self::START
    }
}