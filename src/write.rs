use ::instruction::{Instruction, Decoration};
use ::parse::RawInstruction;
use ::desc::Op;

#[derive(Default)]
struct StringBuilder {
    words: Vec<u32>,
    bytes: Vec<u8>,
}

impl StringBuilder {
    /// Turns a `String` into a padded list of 32-bits words
    pub fn to_words(string: String) -> Vec<u32> {
        let mut builder: StringBuilder = Default::default();

        for byte in string.as_bytes() {
            builder.push_byte(*byte);
        }

        builder.push_byte(0);
        while builder.bytes.len() != 0 {
            builder.push_byte(0);
        }

        builder.words
    }

    fn push_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
        if self.bytes.len() == 4 {
            unsafe {
                let arr = self.bytes.as_ptr() as *const [u8; 4];
                self.words.push(::std::mem::transmute(*arr));
            }

            self.bytes.clear();
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/inst_writer.rs"));
