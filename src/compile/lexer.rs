use std::fs::File;
use std::path::Path;
use crate::compile::token::{
    parse_token,
    TokenStream
};
use crate::fs::buffer::{
    Buffer,
    SimpleBuffer
};
use crate::fs::source::SourceFile;

/// Mode for lexer:
///   - Plain for normal;
///   - DoubleQuoted for literal string quoted by `"`
///   - TripleQuoted for literal text block quoted by `"""`
#[derive(PartialEq)]
pub enum ScanMode {
    Plain,
    DoubleQuoted,
    TripleQuoted,
}

/// Number separator, i.e. `1_000_000` refers to 1,000,000 literally.
const NUMBER_SEPARATOR: char = '_';

/// Lexer for a single file.
pub struct Lexer {
    /// Source file.
    source: SourceFile,
    /// Line number.
    line: usize,
    /// Lexer status.
    status: Status,
    pub token_stream: TokenStream,
}

impl Lexer {

    pub fn new(source: &str) -> Self {
        let path = Path::new(source);
        Self {
            source: SourceFile::File(path.to_str().unwrap().to_string()),
            line: 0,
            status: Status::default(),
            token_stream: TokenStream::new(),
        }
    }

    /// Rewrite in the future.
    #[allow(unused_variables,unused_mut)]
    pub fn read_as_token_stream(&mut self) {
        // let offset: usize;
        // offset = buffer.read(&mut file).unwrap();
        //
        // let mut str_buf: Vec<char> = Vec::with_capacity(1 << 5);
        //
        // let mut token_stream = TokenStream::new();
        //
        // // TODO offset should have been replaced by len.
        // for i in 0 .. offset + 1 {
        //     let ch : char = buffer.get_char(i);
        //
        //     match self.get_pattern() {
        //         LexerPattern::Plain => {
        //             // Alphabetic or digit ??
        //             if ch.is_alphanumeric() || ch == '_' {
        //                 if str_buf.last().is_some() && str_buf.last().unwrap().is_ascii_punctuation() {
        //                     put_token_stream(&mut token_stream, &mut str_buf);
        //                     str_buf.push(ch);
        //                 } else {
        //                     str_buf.push(ch);
        //                 }
        //             } else if '"' == ch {
        //                 self.set_pattern(LexerPattern::DoubleQuoted);
        //             } else if ch.is_whitespace() {
        //                 if str_buf.len() > 0 {
        //                     put_token_stream(&mut token_stream, &mut str_buf);
        //                     str_buf.clear();
        //                 }
        //             } else if ch.is_ascii_punctuation() && ch != '_' {
        //                 if str_buf.len() > 0 {
        //                     put_token_stream(&mut token_stream, &mut str_buf);
        //                     str_buf.clear();
        //                 }
        //                 str_buf.push(ch);
        //             } else {
        //                 if str_buf.len() > 0 {
        //                     put_token_stream(&mut token_stream, &mut str_buf);
        //                     str_buf.clear();
        //                 }
        //             }
        //         },
        //         // Double quoted
        //         //   All characters are appended, expect another quote.
        //         LexerPattern::DoubleQuoted => {
        //             if '"' == ch {
        //                 put_token_stream(&mut token_stream, &mut str_buf);
        //                 self.set_pattern(LexerPattern::Plain);
        //             } else {
        //                 str_buf.push(ch);
        //             }
        //         },
        //     }
        //
        // }

        let mut file = File::open(&self.source.get_path()).unwrap();
        let mut token_stream = TokenStream::new();
        let mut buffer: SimpleBuffer = SimpleBuffer::default();

        let mut offset: usize;
        let mut c_curr: char;
        let mut str_buf: Vec<char> = Vec::with_capacity(1 << 5);

        #[allow(unused_labels)]
        'read: loop {
            let offset = buffer.read(&mut file).unwrap();
            if offset == 0 {
                break;
            }

            log::trace!("range = {}", buffer.range());
            // log::trace!("buffer = {}", buffer);
            // log::info!("array = {}", String::from_utf8(buffer.get_read_array().to_vec()).unwrap());

            'tokenize: for _i in 0 .. buffer.range() {
                c_curr = buffer.get_char(_i);
                log::trace!("c_curr at ({}) is: {}", _i, c_curr);

                match self.status.mode {
                    ScanMode::Plain => {
                        c_curr;

                    }
                    ScanMode::DoubleQuoted => {
                        if buffer.get_char(_i) == '"' {
                            if str_buf.ends_with(&['\\', '\\']) {
                                str_buf.push(c_curr);
                            } else {
                                put_token_stream(&mut token_stream, &mut str_buf);
                                self.status.mode = ScanMode::Plain;
                            }
                        } else {
                            str_buf.push(c_curr);
                        }
                    }
                    ScanMode::TripleQuoted => {

                    }
                }
            }
            buffer.forward(0);
        }


        log::debug!("token_stream = {:?}", &token_stream);
        self.token_stream = token_stream;
    }

    fn get_pattern(&self) -> &ScanMode {
        &self.status.mode
    }

    fn set_pattern(&mut self, mode: ScanMode) {
        self.status.mode = mode
    }

}

/// Status of lexer.
struct Status {
    pub mode: ScanMode,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            mode: ScanMode::Plain,
        }
    }
}

fn put_token_stream(token_stream: &mut TokenStream, str_buf: &mut Vec<char>) {
    if str_buf.len() > 0 {
        let str = str_buf.iter().collect::<String>();
        token_stream.put(parse_token(&str).unwrap());
        str_buf.clear();
    }
}

fn scan_comment() {
}