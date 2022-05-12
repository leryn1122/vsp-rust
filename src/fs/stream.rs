pub trait InputStream {



}

//============================================================================//

pub struct FileInputStream<'a> {
    pub buf: Option<char>,
    pub index: usize,
    // pub streams: StaticVec<Peekable<Chars<'a>>>,
}

impl InputStream for FileInputStream {



}

impl FileInputStream {
    fn default() -> Self {
        Self {
            buf: Option::None,
            index: 0,
            // streams: ,
        }
    }

}


