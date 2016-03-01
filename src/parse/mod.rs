// Copyright 2016 James Miller
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{self, fmt, error};
use std::io;

use desc::Id;

mod parser;
mod read;

pub use self::read::Reader;
pub use self::parser::parse_raw_instruction;


#[derive(Clone, Debug)]
pub struct RawInstruction {
    pub opcode: u16,
    pub params: Vec<u32>
}

#[derive(Clone, Debug)]
pub struct Header {
    pub version: (u8, u8),
    pub generator_id: u32,
    pub id_bound: u32
}


pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    DuplicateId(Id, usize),
    UnknownOpcode(u16),
    IdOutOfRange(Id),
    InvalidParamValue(u32, &'static str),
    InstructionTooShort,
    IoError(io::Error),
    InvalidMagicNumber(u32)
}

impl From<io::Error> for ParseError {
    fn from(e: io::Error) -> ParseError {
        ParseError::IoError(e)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;

        try!(f.write_str("Error while parsing: "));
        match *self {
            DuplicateId(id, idx) => {
                write!(f, "duplicate definition of id `{:?}` at instruction {}",
                       id, idx)
            }
            UnknownOpcode(op) => {
                write!(f, "unknown opcode value `{}`", op)
            }
            IdOutOfRange(id) => {
                write!(f, "id `{:?}` is outside valid range", id)
            }
            InvalidParamValue(val, ty) => {
                write!(f, "invalid value `{}` for parameter of type {}",
                       val, ty)
            }
            InstructionTooShort => f.write_str("instruction is too short"),
            IoError(ref e) => fmt::Display::fmt(e, f),
            InvalidMagicNumber(n) => {
                write!(f, "invalid magic number: {:#08x}", n)
            }
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        use self::ParseError::*;
        match *self {
            DuplicateId(_, _) => "duplicate id definition",
            UnknownOpcode(_) => "unknown instruction opcode",
            IdOutOfRange(_) => "id outside valid range",
            InvalidParamValue(_, _) => "parameter value not valid for type",
            InstructionTooShort => "instruction is too short",
            IoError(ref e) => e.description(),
            InvalidMagicNumber(_) => "invalid magic number"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::ParseError::*;
        match *self {
            IoError(ref e) => Some(e),
            _ => None
        }
    }
}
