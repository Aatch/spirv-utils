// Copyright 2016 James Miller
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use std::io::{Read};

use super::{Result, RawInstruction, Header};

const MAGIC_NUMBER : u32 = 0x07230203;

pub struct Reader<R: ?Sized + Read> {
    need_swap: bool,
    is_eof: bool,
    read_header: bool,
    reader: R
}

impl<R: Read> Reader<R> {
    pub fn new(mut reader: R) -> Result<Reader<R>> {
        let mut word : u32 = 0;
        unsafe {
            let buf : &mut [u8;4] = std::mem::transmute(&mut word);
            try!(reader.read(buf));
        }

        let need_swap = if word == MAGIC_NUMBER {
            false
        } else if word.swap_bytes() == MAGIC_NUMBER {
            true
        } else {
            return Err(super::ParseError::InvalidMagicNumber(word));
        };

        Ok(Reader {
            need_swap: need_swap,
            is_eof: false,
            read_header: false,
            reader: reader
        })
    }
}

impl<R: ?Sized + Read> Reader<R> {
    fn read_word(&mut self) -> Result<u32> {
        if self.is_eof { return Ok(0); }

        let mut word : u32 = 0;
        unsafe {
            let buf : &mut [u8;4] = std::mem::transmute(&mut word);
            let n = try!(self.reader.read(buf));
            if n != 4 {
                self.is_eof = true;
                return Ok(0);
            }
        }

        if self.need_swap {
            Ok(word.swap_bytes())
        } else {
            Ok(word)
        }
    }

    pub fn read_header(&mut self) -> Result<Header> {
        assert!(!self.read_header, "Already read header");
        let version = try!(self.read_word());
        let major = (version >> 16) as u8;
        let minor = ((version >> 8) & 0xFF) as u8;

        let generator_id = try!(self.read_word());
        let id_bound = try!(self.read_word());

        // Skip reserved word
        try!(self.read_word());
        self.read_header = true;

        Ok(Header {
            version: (major, minor),
            generator_id: generator_id,
            id_bound: id_bound
        })
    }

    pub fn skip_header(&mut self) -> Result<()> {
        if !self.read_header {
            self.read_header().map(|_| ())
        } else {
            Ok(())
        }
    }

    pub fn read_instruction(&mut self) -> Result<Option<RawInstruction>> {
        assert!(self.read_header, "Header needs to be read");
        let op = try!(self.read_word());
        if self.is_eof { return Ok(None); }

        let code = (op & 0xFFFF) as u16;
        let count = op >> 16;
        let mut params = Vec::with_capacity(count as usize);

        for _ in 1..count {
            let p = try!(self.read_word());
            params.push(p);
        }

        Ok(Some(RawInstruction {
            opcode: code,
            params: params
        }))
    }
}
