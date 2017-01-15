// Copyright 2016 James Miller
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate lalrpop_util;

use std::borrow::{Cow};
use std::env;
use std::fs::File;
use std::io::{Result, BufRead, BufReader, Read, Write};
use std::path::Path;

use lalrpop_util::ParseError;

pub mod util {
    pub mod codegen;
    pub mod parser;
}

use util::codegen::CodeFile;
use util::parser::parse_Description;

#[derive(Debug)]
pub struct Group {
    name: String,
    instructions: Vec<Instruction>
}

#[derive(Debug)]
pub struct Instruction {
    opcode: u16,
    name: String,
    params: Vec<Param>,
    group: Option<String>
}

#[derive(Debug)]
pub struct Param {
    name: String,
    ty: ParamTy,
}

#[derive(Debug)]
pub enum ParamTy {
    Single(Ty, bool),
    Repeat(Ty),
    RepeatMany(Vec<Ty>)
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=desc/core.desc");
    println!("cargo:rerun-if-changed=util/parser.rs");

    let dest = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&dest);

    let mut input = File::open("desc/core.desc").unwrap();

    let mut buf = String::new();
    input.read_to_string(&mut buf);

    let instructions = parse_Description(&buf);

    match instructions {
        Ok(instructions) => {
            let instructions : Vec<_> = instructions.into_iter().flat_map(|group| {
                let name = group.name;
                group.instructions.into_iter().map(move |mut i| {
                    i.group = Some(name.clone());
                    i
                })
            }).collect();

            let insts_output = CodeFile::create(&dest.join("insts.rs"));
            gen_insts(&instructions, insts_output).unwrap();

            let parser_output = CodeFile::create(&dest.join("inst_parser.rs"));
            gen_parser(&instructions, parser_output).unwrap();

            let writer_output = CodeFile::create(&dest.join("inst_writer.rs"));
            gen_writer(&instructions, writer_output).unwrap();
        }
        Err(e) => {
            let mut stderr = std::io::stderr();

            let _ = writeln!(stderr, "Error parsing core.desc: {:?}", e);
            std::process::exit(1);
        }
    }
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

fn gen_writer(insts: &[Instruction], mut dest: CodeFile) -> Result<()> {
    try!(dest.start_block("pub fn to_bytecode(inst: Instruction) -> Vec<u32> {"));
    try!(dest.start_block("let mut raw = match inst {"));

    for inst in insts {
        if inst.params.len() == 0 {
            try!(dest.write_line(&format!(
                "Instruction::{name} => RawInstruction {{ opcode: Op::{name} as u16, params: vec![] }},", name=inst.name)));
            continue;
        }

        let param_list = inst.params.iter()
            .map(|param| normalize_name(&param.name))
            .collect::<Vec<_>>()
            .join(", ");

        try!(dest.start_block(&format!("Instruction::{} {{{}}} => {{", inst.name, param_list)));
        try!(dest.write_line("let mut _params : Vec<u32> = Vec::new();"));

        for param in &inst.params {
            let name = format!("{}", normalize_name(&param.name));

            match param.ty {
                ParamTy::Single(ref ty, opt) => {
                    let name = match *ty {
                        Ty::Id | Ty::ResultId | Ty::ResultType | Ty::TypeId | Ty::ValueId => {
                            try!(dest.write_line(&format!("if {name}.0 != 0 {{
                                _params.push({name}.0);
                            }}", name=name)));
                            continue;
                        },
                        Ty::ImageOperands => name,
                        _ if opt => format!("{}.unwrap()", name),
                        _ => name,
                    };
                    try!(dest.write_line(&ty.push_words("_params", name)));
                },
                ParamTy::Repeat(ref ty) => {
                    try!(dest.start_block(&format!("for elem in {}.into_iter() {{", name)));
                    try!(dest.write_line(&ty.push_words("_params", "(*elem)".into())));
                    try!(dest.end_block("}"));
                },
                ParamTy::RepeatMany(ref tys) => {
                    try!(dest.start_block(&format!("for elem in {}.into_iter() {{", name)));

                    for (ty, i) in tys.iter().zip(0..tys.len()) {
                        let name = format!("(elem.{})", i);
                        try!(dest.write_line(&ty.push_words("_params", name)));
                    }

                    try!(dest.end_block("}"));
                },
            }
        }

        try!(dest.write_line(&format!("RawInstruction {{ opcode: Op::{} as u16, params: _params }}", inst.name)));
        try!(dest.end_block("}"));
    }

    try!(dest.write_line("_ => unimplemented!()"));
    try!(dest.end_block("};"));

    try!(dest.write_line("let first_word: [u16; 2] = [raw.opcode, 1 + raw.params.len() as u16];"));
    try!(dest.write_line("let mut res = vec![unsafe { ::std::mem::transmute::<[u16; 2], u32>(first_word) }];"));
    try!(dest.write_line("res.append(&mut raw.params);"));

    try!(dest.write_line("res"));
    dest.end_block("}")
}

fn normalize_name<'a>(s: &'a str) -> Cow<'a, str> {
    if s.contains('-') {
        s.replace("-", "_").into()
    } else {
        s.into()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Ty {
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

    pub fn push_words(&self, vec: &'static str, name: String) -> String {
        use self::Ty::*;
        match *self {
            Id | ResultId | ResultType | TypeId | ValueId =>
                format!("{}.push({}.0);", vec, name),
            FPFastMathMode | MemoryAccess | FunctionControl | LoopControl | SelectionControl =>
                format!("{}.push({}.bits());", vec, name),
            ExecutionMode =>
                format!("{}.push({}.to_desc() as u32);", vec, name),
            Decoration =>
                format!("{vec}.push({name}.to_desc() as u32);
                match {name} {{
                    Decoration::SpecId(n) |
                    Decoration::ArrayStride(n) |
                    Decoration::MatrixStride(n) |
                    Decoration::Stream(n) |
                    Decoration::Location(n) |
                    Decoration::Component(n) |
                    Decoration::Index(n) |
                    Decoration::Binding(n) |
                    Decoration::DescriptorSet(n) |
                    Decoration::Offset(n) |
                    Decoration::XfbBuffer(n) |
                    Decoration::XfbStride(n) |
                    Decoration::InputAttachmentIndex(n) |
                    Decoration::Alignment(n) => {{
                        {vec}.push(n as u32);
                    }},
                    Decoration::BuiltIn(b) => {{
                        {vec}.push(b as u32);
                    }}
                    Decoration::FuncParamAttr(attr) => {{
                        {vec}.push(attr as u32);
                    }}
                    Decoration::FPRoundingMode(attr) => {{
                        {vec}.push(attr as u32);
                    }}
                    Decoration::FPFastMathMode(attr) => {{
                        {vec}.push(attr.bits());
                    }}
                    Decoration::LinkageAttributes(name, ty) => {{
                        {vec}.append(&mut StringBuilder::to_words(name));
                        {vec}.push(ty as u32);
                    }}
                    _ => {{}}
                }}", vec=vec, name=name),
            ImageOperands =>
                format!("{}.push({}.to_desc().bits());", vec, name),
            String =>
                format!("{}.append(&mut StringBuilder::to_words({}));", vec, name),
            Number =>
                format!("{}.push({});", vec, name),
            _ =>
                format!("{}.push({} as u32);", vec, name)
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
