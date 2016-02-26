use std;
use std::io::{Result, Read};

use super::{RawInstruction};
use module::Header;

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
            panic!("Invalid Magic Word: {:#x}", word)
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

/*
pub fn parse_instruction(inst: RawInstruction) -> Instruction {
    use desc::{self, Op, ExecutionMode};

    struct ParamParser {
        idx: usize,
        params: Vec<u32>
    }

    impl ParamParser {
        fn parse_word(&mut self) -> u32 {
            if self.idx >= self.params.len() {
                0
            } else {
                let idx = self.idx;
                self.idx += 1;
                self.params[idx]
            }
        }
        fn parse_id(&mut self) -> desc::Id {
            if self.idx >= self.params.len() {
                return desc::Id(0);
            }
            let id = self.params[self.idx];
            self.idx += 1;
            desc::Id(id)
        }

        fn parse_string(&mut self) -> String {
            if self.idx >= self.params.len() {
                return String::new();
            }

            let mut buf = Vec::new();
            'words: for w in &self.params[self.idx..] {
                self.idx += 1;
                let mut w = *w;
                // Loop through each byte in the word
                for _ in 0..4 {
                    // If we see a zero byte, then we're done with
                    // the string
                    if (w & 0xFF) == 0 {
                        break 'words;
                    }
                    let b = (w & 0xFF) as u8;
                    buf.push(b);
                    w = w >> 8;
                }
            }

            String::from_utf8(buf).unwrap()
        }

        fn remaining(&self) -> usize {
            self.params.len() - self.idx
        }
    }

    let op = if let Some(op) = Op::from(inst.opcode) {
        op
    } else {
        return Instruction::Unknown(inst.opcode, inst.params.into_boxed_slice());
    };

    let mut p = ParamParser {
        idx: 0,
        params: inst.params
    };

    macro_rules! parse_inst(
        ($p:ident, $op:ident,) => (

        );
        ($p:ident, $op:ident, $name:ident, $($rest:tt)*) => (
            if $op == Op::$name {
                return Instruction::$name;
            } else {
                parse_inst!($p, $op, $($rest)*);
            }
        );
        ($p:ident, $op:ident, $name:ident($($param:tt),*), $($rest:tt)*) => (
            if $op == Op::$name {
                return Instruction::$name($(parse_param!($p, $param)),*);
            } else {
                parse_inst!($p, $op, $($rest)*);
            }
        )
    );

    macro_rules! parse_param (
        ($p:ident, Id) => ($p.parse_id());
        ($p:ident, String) => ($p.parse_string());
        ($p:ident, u32) => ($p.parse_word());
        ($p:ident, rest_Id) => ({
            let rem = $p.remaining();
            let mut v = Vec::with_capacity(rem);
            for _ in 0..rem {
                v.push($p.parse_id());
            }
            v.into_boxed_slice()
        });
        ($p:ident, rest_u32) => ({
            let rem = $p.remaining();
            let mut v = Vec::with_capacity(rem);
            for _ in 0..rem {
                v.push($p.parse_word());
            }
            v.into_boxed_slice()
        });
        ($p:ident, $other:ident) => (
            desc::$other::from($p.parse_word())
                .expect(concat!("Invalid ", stringify!($other)))
        );
    );

    parse_inst!(
        p, op,
        Nop,
        Undef(Id, Id),
        SourceContinued(String),
        SourceExtension(String),
        Name(Id, String),
        MemberName(Id, u32, String),
        String(Id, String),
        Line(Id, u32, u32),
        Extension(String),
        ExtInstImport(Id, String),
        ExtInst(Id, Id, Id, u32, rest_Id),
        MemoryModel(AddressingModel, MemoryModel),
        EntryPoint(ExecutionModel, Id, String, rest_Id),
        Capability(Capability),
    );

    match op {
        Op::Source => {
            let lang = desc::SrcLang::from(p.parse_word())
                .unwrap_or(desc::SrcLang::Unknown);
            let version = p.parse_word();
            let file = p.parse_id();
            let mut src = None;
            if file != desc::Id(0) {
                src = Some(p.parse_string())
            }

            Instruction::Source(lang, version, file, src)
        }
        Op::ExecutionMode => {
            let id = p.parse_id();
            let mode = p.parse_word();

            macro_rules! parse_mode (
                ($mode:expr, $($name:ident = $val:expr),*) => (
                    match $mode {
                        $($val => ExecutionMode::$name,)*
                        0 => {
                            let n = p.parse_word();
                            ExecutionMode::Invocations(n)
                        }
                        17 => {
                            let x = p.parse_word();
                            let y = p.parse_word();
                            let z = p.parse_word();
                            ExecutionMode::LocalSize(x, y, z)
                        }
                        18 => {
                            let x = p.parse_word();
                            let y = p.parse_word();
                            let z = p.parse_word();
                            ExecutionMode::LocalSizeHint(x, y, z)
                        }
                        26 => {
                            let n = p.parse_word();
                            ExecutionMode::OutputVertices(n)
                        }
                        30 => {
                            let n = p.parse_word();
                            ExecutionMode::VecTypeHint(n)
                        }
                        m => {
                            panic!("Invalid Execution mode '{}'", m)
                        }
                    }
                )
            );

            let mode = parse_mode!(
                mode,
                SpacingEqual = 1,
                SpacingFractionalEven = 2,
                SpacingFractionalOdd = 3,
                VertexOrderCw = 4,
                VertexOrderCcw = 5,
                PixelCenterInteger = 6,
                OriginUpperLeft = 7,
                OriginLowerLeft = 8,
                EarlyFragmentTests = 9,
                PointMode = 10,
                Xfb = 11,
                DepthReplacing = 12,
                DepthGreater = 13,
                DepthLess = 14,
                DepthUnchanged = 15,
                InputPoints = 19,
                InputLines = 20,
                InputLinesAdjacency = 21,
                Triangles = 22,
                InputTrianglesAdjacency = 23,
                Quads = 24,
                IsoLines = 25,
                OutputPoints = 27,
                OutputLineStrip = 28,
                OutputTriangleStrip = 29,
                ContractionOff = 31
            );

            Instruction::ExecutionMode(id, mode)
        }
        _ => Instruction::Unknown(inst.opcode, p.params.into_boxed_slice())
    }
}
*/
