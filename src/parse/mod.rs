use std::{self, fmt, error};
use std::io;

use desc::Id;

mod parser;
mod read;


pub use self::read::Reader;
pub use self::parser::parse_raw_instruction;

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    DuplicateId(Id, usize),
    UnknownOpcode(u16),
    IdOutOfRange(Id),
    InvalidParamValue(u32, &'static str),
    InstructionTooShort,
    IoError(io::Error)
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
        match self {
            &DuplicateId(id, idx) => {
                write!(f, "duplicate definition of id `{:?}` at instruction {}",
                       id, idx)
            }
            &UnknownOpcode(op) => {
                write!(f, "unknown opcode value `{}`", op)
            }
            &IdOutOfRange(id) => {
                write!(f, "id `{:?}` is outside valid range", id)
            }
            &InvalidParamValue(val, ty) => {
                write!(f, "invalid value `{}` for parameter of type {}",
                       val, ty)
            }
            &InstructionTooShort => f.write_str("instruction is too short"),
            &IoError(ref e) => fmt::Display::fmt(e, f)
        }
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        use self::ParseError::*;
        match self {
            &DuplicateId(_, _) => "duplicate id definition",
            &UnknownOpcode(_) => "unknown instruction opcode",
            &IdOutOfRange(_) => "id outside valid range",
            &InvalidParamValue(_, _) => "parameter value not valid for type",
            &InstructionTooShort => "instruction is too short",
            &IoError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::ParseError::*;
        match self {
            &IoError(ref e) => Some(e),
            _ => None
        }
    }
}
