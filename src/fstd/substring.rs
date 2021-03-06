pub trait Substring {
    fn substring(&self, start_index: usize, end_index: usize) -> &str;
}

#[allow(deprecated)]
impl Substring for str {
    fn substring(&self, start_index: usize, end_index: usize) -> &str {
        if end_index <= start_index {
            return "";
        }

        let mut indices = self.char_indices();

        let obtain_index = |(index, _char)| index;
        let str_len = self.len();

        unsafe {
            // SAFETY: Since `indices` iterates over the `CharIndices` of `self`, we can guarantee
            // that the indices obtained from it will always be within the bounds of `self` and they
            // will always lie on UTF-8 sequence boundaries.
            self.slice_unchecked(
                indices.nth(start_index).map_or(str_len, &obtain_index),
                indices
                    .nth(end_index - start_index - 1)
                    .map_or(str_len, &obtain_index),
            )
        }
    }
}