// Copyright 2016 James Miller
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;

pub struct LineParser<'a> {
    line: &'a [u8]
}

impl<'a> LineParser<'a> {
    pub fn new(line: &'a str) -> LineParser<'a> {
        let line = line.trim_right().as_bytes();

        LineParser {
            line: line
        }
    }

    pub fn is_eol(&self) -> bool {
        self.line.len() == 0
    }

    pub fn eat(&mut self, c: char) -> bool {
        let len = c.len_utf8();
        if self.line.len() < len {
            return false;
        }
        if len == 1 {
            let c = c as u8;
            if self.line[0] == c {
                self.bump(1);
                return true;
            }
        } else {
            panic!("Non-ASCII character given!");
        }

        false
    }

    pub fn skip_whitespace(&mut self) {
        while self.line.len() > 0 && is_whitespace(self.line[0]) {
            self.bump(1);
        }
    }

    pub fn parse_number(&mut self) -> Option<u16> {
        self.skip_whitespace();


        let mut n = 0;
        let mut any = false;

        while self.line.len() > 0 {
            let c = self.line[0];
            if is_digit(c) {
                self.bump(1);
                any = true;
                n *= 10;
                n += (c - b'0') as u16;
            } else {
                break;
            }
        }

        if any {
            Some(n)
        } else {
            None
        }
    }

    pub fn parse_word(&mut self) -> Option<&'a str> {
        self.skip_whitespace();

        let mut i = 0;
        while i < self.line.len() {
            let c = self.line[i];
            if (c >= b'a' && c <= b'z') ||
               (c >= b'A' && c <= b'Z') ||
               (c == b'_') ||
               (i > 0 && (is_digit(c) || c == b'-')) {

                i += 1;
            } else {
               break;
            }
        }

        if i == 0 {
            None
        } else {
            let word = str::from_utf8(&self.line[0..i]).unwrap();
            self.line = &self.line[i..];
            Some(word)
        }
    }

    pub fn bump(&mut self, n: usize) {
        let n = if n > self.line.len() {
            self.line.len()
        } else {
            n
        };

        self.line = &self.line[n..];
    }
}

fn is_whitespace(c: u8) -> bool {
    match c {
        b' ' |
        b'\t' |
        b'\n' |
        b'\r' => true,
        _ => false
    }
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}
