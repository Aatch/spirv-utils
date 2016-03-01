// Copyright 2016 James Miller
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::{Result, Write, BufWriter};
use std::fs::File;
use std::path::Path;

pub struct CodeFile {
    file: BufWriter<File>,
    indent: u32,
    need_indent: bool
}

impl CodeFile {
    pub fn create<P: AsRef<Path>>(path: P) -> CodeFile {
        let file = File::create(path).unwrap();
        let file = BufWriter::new(file);

        CodeFile {
            file: file,
            indent: 0,
            need_indent: false
        }
    }

    pub fn start_block(&mut self, text: &str) -> Result<()> {
        try!(self.write_line(text));
        self.indent += 1;
        Ok(())
    }

    pub fn new_block(&mut self, text: &str) -> Result<()> {
        if self.indent > 0 {
            self.indent -= 1;
        }
        try!(self.write_line(text));
        self.indent += 1;
        Ok(())
    }

    pub fn end_block(&mut self, text: &str) -> Result<()> {
        if self.indent > 0 {
            self.indent -= 1;
        }
        try!(self.write_line(text));
        if self.indent == 0 {
            self.file.flush();
        }
        Ok(())
    }

    pub fn write_line(&mut self, text: &str) -> Result<()> {
        self.write_indent();
        try!(self.file.write_all(text.as_bytes()));
        try!(self.file.write_all(b"\n"));
        self.need_indent = true;
        Ok(())
    }

    pub fn write(&mut self, text: &str) -> Result<()> {
        self.write_indent();
        self.file.write_all(text.as_bytes())
    }

    fn write_indent(&mut self) -> Result<()> {
        if self.need_indent {
            for _ in 0..self.indent {
                try!(self.file.write_all(b"    "));
            }
            self.need_indent = false;
        }
        Ok(())
    }
}
