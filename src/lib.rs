// Copyright 2016 James Miller
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::Read;
use std::path::Path;

pub mod desc;
pub mod instruction;
pub mod parse;
pub mod write;

use desc::Id;
use instruction::Instruction;
use parse::Result;

/**
 * Minimal representation of a SPIR-V module.
 */
pub struct RawModule {
    instructions: Vec<Instruction>,
    def_map: Box<[usize]>,
    use_map: Box<[Vec<usize>]>
}

impl RawModule {

    /**
     * Load a module from a file
     */
    pub fn load_module<P: AsRef<Path>>(path: P) -> Result<RawModule> {
        let file = try!(std::fs::File::open(path));

        RawModule::read_module(file)
    }

    /**
     * Read a module
     */
    pub fn read_module<R: Read>(reader: R) -> Result<RawModule> {
        let mut reader = try!(parse::Reader::new(reader));

        let header = try!(reader.read_header());

        let ids = header.id_bound as usize;
        let mut def_map = (vec![!0; ids]).into_boxed_slice();
        let mut use_map = (vec![Vec::new(); ids]).into_boxed_slice();

        let mut instructions = Vec::with_capacity(ids);

        while let Some(raw_inst) = try!(reader.read_instruction()) {
            let inst = try!(parse::parse_raw_instruction(raw_inst));
            let inst_idx = instructions.len();

            if let Some(id) = inst.defines_value() {
                let idx = id.0 as usize;
                def_map[idx] = inst_idx;
            } else if let Some(id) = inst.defines_type() {
                let idx = id.0 as usize;
                def_map[idx] = inst_idx;
            }

            let uses = inst.uses();
            for id in uses {
                let idx = id.0 as usize;
                if idx == 0 { continue; }
                use_map[idx].push(inst_idx);
            }

            instructions.push(inst);
        }

        Ok(RawModule {
            instructions: instructions,
            def_map: def_map,
            use_map: use_map
        })
    }

    /**
     * Gets the instructions in the module
     */
    pub fn instructions<'a>(&'a self) -> &'a [Instruction] {
        &self.instructions[..]
    }

    /**
     * Gets the index of the instruction that defines the given id, if
     * any
     */
    pub fn def_index<I: Into<Id>>(&self, id: I) -> Option<usize> {
        let idx = id.into().0 as usize;
        if idx == 0 { return None; }

        self.def_map.get(idx).and_then(|&idx| {
            if idx == !0 {
                None
            } else {
                Some(idx)
            }
        })
    }

    /**
     * Gets the indices of the instructions that use the given id
     */
    pub fn use_indices<'a, I: Into<Id>>(&'a self, id: I) -> Option<&'a [usize]> {
        let idx = id.into().0 as usize;
        if idx == 0 { return None; }

        self.use_map.get(idx).map(|indices| {
            &indices[..]
        })
    }

    /**
     * Gets the instruction that defines the given Id, if any
     */
    pub fn def<'a, I: Into<Id>>(&'a self, id: I) -> Option<&'a Instruction> {
        self.def_index(id).map(|idx| {
            &self.instructions[idx]
        })
    }

    /**
     * Returns an iterator over the uses of the given Id
     */
    pub fn uses<'a, I: Into<Id>>(&'a self, id: I) -> Uses<'a> {
        let indices = self.use_indices(id).unwrap_or(&[]);

        Uses {
            instructions: self.instructions(),
            indices: indices
        }
    }
}

pub struct Uses<'a> {
    instructions: &'a [Instruction],
    indices: &'a [usize],
}

impl<'a> Iterator for Uses<'a> {
    type Item = &'a Instruction;

    fn next(&mut self) -> Option<&'a Instruction> {
        if self.indices.len() == 0 {
            return None;
        }

        let idx = self.indices[0];
        self.indices = &self.indices[1..];

        self.instructions.get(idx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.indices.len();
        (len, Some(len))
    }
}
