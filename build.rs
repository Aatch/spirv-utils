// Copyright 2016 James Miller
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::{Cow};
use std::env;
use std::fs::File;
use std::io::{Result, BufRead, BufReader, Read, Write};
use std::path::Path;

pub mod util {
    pub mod codegen;
    pub mod line_parser;
}

use util::codegen::CodeFile;
use util::line_parser::LineParser;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=desc/core.desc");

    let dest = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&dest);

    let input = File::open("desc/core.desc").unwrap();
    let mut input = BufReader::new(input);

    let mut cur_group = None;

    let mut instructions = Vec::new();

    for line in input.lines() {
        let line = line.unwrap();
        let line : &str = line.trim_right();
        if line == "" { continue; }

        if line.starts_with("#") {
            // Skip comments
            continue;
        } else if line.starts_with("group") {
            let group_name = &line[5..];
            let group_name = group_name.trim();
            cur_group = Some(group_name.to_owned());
        } else {
            let mut inst = parse_line(line);
            inst.group = cur_group.clone();

            instructions.push(inst);
        }
    }

    let insts_output = CodeFile::create(&dest.join("insts.rs"));
    gen_insts(&instructions, insts_output).unwrap();

    let parser_output = CodeFile::create(&dest.join("inst_parser.rs"));
    gen_parser(&instructions, parser_output).unwrap();
}

fn gen_insts(insts: &[Instruction], mut dest: CodeFile) -> Result<()> {

    // Generate the definition, each instruction is a struct variant
    try!(dest.write_line("#[derive(Clone, Debug)]"));
    try!(dest.start_block("pub enum Instruction {"));

    for inst in insts {
        try!(dest.write(&inst.name));
        if inst.params.len() > 0 {
            try!(dest.start_block(" {"));

            for param in &inst.params {
                let name = normalize_name(&param.name);
                let ty = param.ty.rust_type_name();
                try!(dest.write_line(&format!("{}: {},", name, ty)));
            }

            try!(dest.end_block("},"));
        } else {
            try!(dest.write_line(","));
        }

    }

    try!(dest.write_line("Unknown(u16, Box<[u32]>)"));


    try!(dest.end_block("}\n\n"));

    // Generate some methods
    try!(dest.start_block("impl Instruction {"));

    fn extract_field<'a, I: IntoIterator<Item=&'a Instruction>>(
        dest: &mut CodeFile, name: &str, pb: bool, ret_ty: &str,
        insts: I, field: &str) -> Result<()> {

        let pb = if pb { "pub " } else { "" };

        try!(dest.start_block(
            &format!("{}fn {}(&self) -> Option<{}> {{", pb, name, ret_ty)));
        try!(dest.write_line("use self::Instruction::*;"));
        try!(dest.start_block("match *self {"));

        let insts = insts.into_iter().filter(|i| {
            for p in &i.params {
                if &p.name[..] == field {
                    return true;
                }
            }
            false
        });

        let field = normalize_name(field);

        let mut first = true;
        for inst in insts {
            if first {
                first = false;
            } else {
                try!(dest.write_line(" |"));
            }

            if inst.params.len() == 1 {
                try!(dest.write(&format!("{} {{ {} }}", inst.name, field)));
            } else {
                try!(dest.write(&format!("{} {{ {}, .. }}", inst.name, field)));
            }
        }
        try!(dest.write_line(&format!(" => Some({}),", field)));
        try!(dest.write_line("_ => None"));

        try!(dest.end_block("}"));
        dest.end_block("}")
    }

    let non_types = insts.iter().filter(|i| {
        let group : Option<&str> = i.group.as_ref().map(|g| &g[..]);
        if group == Some("Type") {
            return false;
        }

        return true;
    }).collect::<Vec<_>>();

    // Generate method for getting the id of the value this instruction defines, if any.
    try!(extract_field(&mut dest, "defines_value_inner", false, "ResultId", non_types.iter().cloned(), "result-id"));
    // Generate method for getting the id of the type of the result of this instruction, if any.
    try!(extract_field(&mut dest, "type_id_of", true, "TypeId", non_types.iter().cloned(), "result-type"));

    let types = insts.iter().filter(|i| {
        let group : Option<&str> = i.group.as_ref().map(|g| &g[..]);
        if group == Some("Type") {
            return true;
        }

        return false;
    });

    // Generate method for getting the id of the type of the instruction defines
    try!(extract_field(&mut dest, "defines_type", true, "TypeId", types, "result-type"));

    // Finally generate a method for getting all the ids used by the instruction
    let users = insts.iter().filter(|i| {
        for p in &i.params {
            if p.name.starts_with("result") {
                continue;
            }

            match p.ty {
                ParamTy::Single(ty, _) |
                ParamTy::Repeat(ty) => {
                    match ty {
                        Ty::Id | Ty::TypeId | Ty::ValueId => return true,
                        _ => ()
                    }
                }
                ParamTy::RepeatMany(ref tys) => {
                    for &ty in tys {
                        match ty {
                            Ty::Id | Ty::TypeId | Ty::ValueId => return true,
                            _ => ()
                        }
                    }
                }
            }
        }

        return false;
    });

    try!(dest.start_block("pub fn uses(&self) -> Vec<Id> {"));
    try!(dest.write_line("use self::Instruction::*;"));
    try!(dest.start_block("match *self {"));

    for u in users {
        let params = u.params.iter().filter(|p| {
            if p.name.starts_with("result") {
                return false;
            }

            match p.ty {
                ParamTy::Single(ty, _) |
                ParamTy::Repeat(ty) => {
                    match ty {
                        Ty::Id | Ty::TypeId | Ty::ValueId => return true,
                        _ => ()
                    }
                }
                ParamTy::RepeatMany(ref tys) => {
                    for &ty in tys {
                        match ty {
                            Ty::Id | Ty::TypeId | Ty::ValueId => return true,
                            _ => ()
                        }
                    }
                }
            }

            false
        }).collect::<Vec<_>>();

        try!(dest.start_block(&format!("{} {{", u.name)));
        let mut i = 0;
        for p in &params {
            let name = normalize_name(&p.name);
            match p.ty {
                ParamTy::Single(..) => {
                    try!(dest.write_line(&format!("{},", name)));
                }
                ParamTy::Repeat(..) |
                ParamTy::RepeatMany(..) => {
                    try!(dest.write_line(&format!("ref {},", name)));
                }
            }
            i += 1;
        }

        if i != u.params.len() {
            try!(dest.write_line(".."));
        }
        try!(dest.new_block("} => {"));
        try!(dest.write_line("let mut ids = Vec::new();"));

        for p in params {
            let name = normalize_name(&p.name);
            match p.ty {
                ParamTy::Single(..) => {
                    try!(dest.write_line(&format!("ids.push({}.into());", name)));
                }
                ParamTy::Repeat(..) => {
                    try!(dest.write_line(&format!("ids.extend({}.iter().map(|i| Id::from(*i)));", name)));
                }
                ParamTy::RepeatMany(ref tys) => {
                    try!(dest.start_block(&format!("for x in {}.iter() {{", name)));
                    for (i, &ty) in tys.iter().enumerate() {
                        match ty {
                            Ty::Id | Ty::TypeId | Ty::ValueId => {
                                try!(dest.write_line(&format!("ids.push(x.{}.into());", i)));
                            }
                            _ => ()
                        }
                    }
                    try!(dest.end_block("}"));
                }
            }
        }
        try!(dest.write_line("ids"));
        try!(dest.end_block("}"));
    }

    try!(dest.write_line("_ => Vec::new()"));

    try!(dest.end_block("}"));
    try!(dest.end_block("}"));

    dest.end_block("}")

}

fn gen_parser(insts: &[Instruction], mut dest: CodeFile) -> Result<()> {
    try!(dest.start_block(
        "pub fn parse_raw_instruction(raw_inst: RawInstruction) -> Result<Instruction> {"));
    try!(dest.start_block(
        "let op = if let Some(op) = desc::Op::from(raw_inst.opcode) {"));
    try!(dest.write_line("op"));
    try!(dest.new_block("} else {"));
    try!(dest.write_line("return Err(ParseError::UnknownOpcode(raw_inst.opcode));"));
    try!(dest.end_block("};"));

    try!(dest.write_line("let mut p = InstructionParser { params: &raw_inst.params };"));

    try!(dest.start_block("let inst = match op {"));

    for inst in insts {
        if inst.params.len() == 0 {
            try!(dest.write_line(&format!(
                "Op::{name} => Instruction::{name},", name=inst.name)));
            continue;
        }

        try!(dest.start_block(&format!("Op::{} => {{", inst.name)));

        for param in &inst.params {
            let name = normalize_name(&param.name);

            if let ParamTy::Single(ty, true) = param.ty {
                if ty.is_id() {
                    let ty_name = ty.rust_type_name(false);
                    try!(dest.write_line(&format!("let mut {} = {}(0);", name, ty_name)));
                    try!(dest.start_block("if p.has_words() {"));
                    try!(dest.write_line(&format!(
                        "{} = try!(p.parse::<{}>());\n", name, ty_name)));
                    try!(dest.end_block("}"));
                    continue;
                }
            }

            if let ParamTy::RepeatMany(ref tys) = param.ty {
                let tys : Vec<_> = tys.iter().map(|ty| ty.rust_type_name(false)).collect();
                let ty = tys.join(", ");
                try!(dest.write_line(&format!(
                    "let mut {} : Vec<({})> = Vec::new();", name, ty)));
                try!(dest.start_block("while p.has_words() {"));
                try!(dest.start_block(&format!("{}.push((", name)));
                for ty in &tys {
                    try!(dest.write_line(&format!("try!(p.parse::<{}>()),", ty)));
                }
                try!(dest.end_block("));"));
                try!(dest.end_block("}"));
                try!(dest.write_line(&format!(
                    "let {name} = {name}.into_boxed_slice();", name=name)));

                continue;
            }

            let ty = param.ty.rust_type_name();

            try!(dest.write_line(&format!("let {} = try!(p.parse::<{}>());", name, ty)));
        }

        try!(dest.start_block(&format!("Instruction::{} {{", inst.name)));
        for param in &inst.params {
            let name = normalize_name(&param.name);
            try!(dest.write_line(&format!("{name}: {name},", name=name)));
        }
        try!(dest.end_block("}"));
        try!(dest.end_block("}"));
    }

    try!(dest.write_line("_ => Instruction::Unknown(op as u16, p.params.to_owned().into_boxed_slice())\n"));

    try!(dest.end_block("};"));
    try!(dest.write_line("Ok(inst)"));

    dest.end_block("}")
}

fn normalize_name<'a>(s: &'a str) -> Cow<'a, str> {
    if s.contains('-') {
        s.replace("-", "_").into()
    } else {
        s.into()
    }
}

#[derive(Debug)]
struct Instruction {
    op: u16,
    name: String,
    params: Vec<Param>,
    group: Option<String>,
}

#[derive(Debug)]
struct Param {
    name: String,
    ty: ParamTy,
}

#[derive(Debug)]
enum ParamTy {
    Single(Ty, bool),
    Repeat(Ty),
    RepeatMany(Vec<Ty>)
}

#[derive(Copy, Clone, Debug)]
enum Ty {
    Id,
    ResultType,
    ResultId,
    TypeId,
    ValueId,
    String,
    Number,
    Bool,
    Decoration,
    ExecutionMode,
    ImageOperands,
    SrcLang,
    ExecutionModel,
    AddressingModel,
    MemoryModel,
    StorageClass,
    Dim,
    SamplerAddressingMode,
    SamplerFilterMode,
    ImageFormat,
    ImageChannelOrder,
    ImageChannelDatatype,
    FPRoundingMode,
    AccessQualifier,
    BuiltIn,
    GroupOperation,
    Capability,
    FPFastMathMode,
    SelectionControl,
    LoopControl,
    FunctionControl,
    MemoryOrdering,
    MemoryAccess,
    KernelProfilingInfo
}

impl Ty {
    pub fn from_str(s: &str) -> Option<Ty> {
        use self::Ty::*;
        let ty = match s {
            "id" => Id,
            "result-type" => ResultType,
            "type-id" => TypeId,
            "result-id" => ResultId,
            "value-id" => ValueId,
            "string" => String,
            "num" => Number,
            "bool" => Bool,

            "decoration" => Decoration,
            "execution-mode" => ExecutionMode,
            "image-operands" => ImageOperands,

            "src-lang" => SrcLang,
            "execution-model" => ExecutionModel,
            "addressing-model" => AddressingModel,
            "memory-model" => MemoryModel,
            "storage-class" => StorageClass,
            "dim" => Dim,
            "sampler-addressing-mode" => SamplerAddressingMode,
            "sampler-filter-mode" => SamplerFilterMode,
            "image-format" => ImageFormat,
            "image-channel-order" => ImageChannelOrder,
            "image-channel-datatype" => ImageChannelDatatype,
            "fp-rounding-mode" => FPRoundingMode,
            "access-qualifier" => AccessQualifier,
            "builtin" => BuiltIn,
            "group-operation" => GroupOperation,
            "capability" => Capability,

            "fp-fast-math-mode" => FPFastMathMode,
            "selection-control" => SelectionControl,
            "loop-control" => LoopControl,
            "function-control" => FunctionControl,
            "memory-ordering" => MemoryOrdering,
            "memory-access" => MemoryAccess,
            "kernel-profiling-info" => KernelProfilingInfo,

            _ => return None
        };

        Some(ty)
    }

    pub fn is_id(&self) -> bool {
        match *self {
            Ty::Id |
            Ty::ResultId |
            Ty::ResultType | Ty::TypeId |
            Ty::ValueId => true,
            _ => false
        }
    }

    pub fn rust_type_name(&self, opt: bool) -> Cow<'static, str> {
        use self::Ty::*;
        if opt {
            let name = self.rust_type_name(false);
            if self.is_id() {
                return name;
            }
            match *self {
                MemoryAccess | ImageOperands => name,
                _ => {
                    let name = format!("Option<{}>", name);
                    name.into()
                }
            }
        } else {
            let name = match *self {
                Id => "Id",
                ResultId => "ResultId",
                ResultType | TypeId => "TypeId",
                ValueId => "ValueId",
                String => "String",
                Number => "u32",
                Bool => "bool",
                Decoration => "Decoration",
                ExecutionMode => "ExecutionMode",
                ImageOperands => "ImageOperands",

                SrcLang => "desc::SrcLang",
                ExecutionModel => "desc::ExecutionModel",
                AddressingModel => "desc::AddressingModel",
                MemoryModel => "desc::MemoryModel",
                StorageClass => "desc::StorageClass",
                Dim => "desc::Dim",
                SamplerAddressingMode => "desc::SamplerAddressingMode",
                SamplerFilterMode => "desc::SamplerFilterMode",
                ImageFormat => "desc::ImageFormat",
                ImageChannelOrder => "desc::ImageChannelOrder",
                ImageChannelDatatype => "desc::ImageChannelDatatype",
                FPRoundingMode => "desc::FPRoundingMode",
                AccessQualifier => "desc::AccessQualifier",
                BuiltIn => "desc::BuiltIn",
                GroupOperation => "desc::GroupOperation",
                Capability => "desc::Capability",
                FPFastMathMode => "desc::FPFastMathMode",
                SelectionControl => "desc::SelectionControl",
                LoopControl => "desc::LoopControl",
                FunctionControl => "desc::FunctionControl",
                MemoryOrdering => "desc::MemoryOrdering",
                MemoryAccess => "desc::MemoryAccess",
                KernelProfilingInfo => "desc::KernelProfilingInfo"
            };

            name.into()
        }
    }
}

impl ParamTy {
    pub fn rust_type_name(&self) -> Cow<'static, str> {
        match *self {
            ParamTy::Single(ty, opt) => ty.rust_type_name(opt),
            ParamTy::Repeat(ty) => {
                let name = format!("Box<[{}]>", ty.rust_type_name(false));
                name.into()
            }
            ParamTy::RepeatMany(ref tys) => {
                let tys : Vec<_> = tys.iter().map(|ty| {
                    ty.rust_type_name(false)
                }).collect();
                let name = format!("Box<[({})]>", tys.join(", "));
                name.into()
            }
        }
    }
}

fn parse_line(line: &str) -> Instruction {
    let mut line = LineParser::new(line);

    let op = line.parse_number().expect("Couldn't parse opcode");
    let name = line.parse_word().expect("Couldn't parse instruction name");

    let mut params = Vec::new();

    line.skip_whitespace();

    while !line.is_eol() {
        let param = parse_param(&mut line);
        params.push(param);
        line.skip_whitespace();
    }

    Instruction {
        op: op,
        name: name.to_owned(),
        params: params,
        group: None
    }
}

fn parse_param(line: &mut LineParser) -> Param {
    let name = line.parse_word().expect("Couldn't parse param name");

    let ty = if line.eat(':') {
        parse_type(line)
    } else {
        let optional = line.eat('?');
        let ty = Ty::from_str(name).expect(&format!("Invalid type `{}`", name));
        ParamTy::Single(ty, optional)
    };

    Param {
        name: name.to_owned(),
        ty: ty
    }
}

fn parse_type(line: &mut LineParser) -> ParamTy {
    if line.eat('[') {
        let ty = line.parse_word().expect("Couldn't parse type");
        let ty = Ty::from_str(ty).expect(&format!("Invalid type `{}`", ty));

        line.skip_whitespace();
        if line.eat(']') {
            return ParamTy::Repeat(ty);
        } else {
            let mut tys = vec![ty];
            while !line.eat(']') {
                let ty = line.parse_word().expect("Couldn't parse type");
                let ty = Ty::from_str(ty).expect(&format!("Invalid type `{}`", ty));
                tys.push(ty);
            }

            return ParamTy::RepeatMany(tys);
        }
    }

    let ty = line.parse_word().expect("Couldn't parse type");
    let ty = Ty::from_str(ty).expect(&format!("Invalid type `{}`", ty));
    let optional = line.eat('?');
    ParamTy::Single(ty, optional)
}
