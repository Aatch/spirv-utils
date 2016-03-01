// Copyright 2016 James Miller
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;

use super::{RawInstruction, ParseError, Result};

use desc::{self, Id, ValueId, TypeId, ResultId, Op};
use instruction::{self, Instruction, Decoration, ExecutionMode, ImageOperands};

include!(concat!(env!("OUT_DIR"), "/inst_parser.rs"));

struct InstructionParser<'a> {
    params: &'a [u32]
}

impl<'a> InstructionParser<'a> {
    fn parse_word(&mut self) -> Result<u32> {
        if self.params.len() > 0 {
            let w = self.params[0];
            self.params = &self.params[1..];
            Ok(w)
        } else {
            Err(ParseError::InstructionTooShort)
        }
    }

    fn has_words(&mut self) -> bool {
        self.params.len() > 0
    }

    fn remaining_words(&self) -> usize {
        self.params.len()
    }

    fn parse<T: ParamParse>(&mut self) -> Result<T> {
        T::parse(self)
    }
}

trait ParamParse : Sized {
    fn parse(p: &mut InstructionParser) -> Result<Self>;
}

impl ParamParse for Id {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        let id = try!(p.parse_word());
        Ok(Id(id))
    }
}

impl ParamParse for ValueId {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        let id = try!(p.parse_word());
        Ok(ValueId(id))
    }
}

impl ParamParse for TypeId {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        let id = try!(p.parse_word());
        Ok(TypeId(id))
    }
}

impl ParamParse for ResultId {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        let id = try!(p.parse_word());
        Ok(ResultId(id))
    }
}

impl ParamParse for String {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        let mut buf: Vec<u8> = Vec::new();
        'words: while p.has_words() {
            let mut w = try!(p.parse_word());
            for _ in 0..4 {
                if (w & 0xFF) == 0 {
                    break 'words;
                }
                let b = (w & 0xFF) as u8;
                buf.push(b);
                w = w >> 8;
            }
        }

        // Do lossy conversion since UTF-8 errors are
        // likely to only be in non-semantic locations
        // anyway.
        let s = String::from_utf8_lossy(&buf);
        Ok(s.into_owned())
    }
}

impl ParamParse for u32 {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        p.parse_word()
    }
}

impl ParamParse for bool {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        let v = try!(p.parse_word());
        Ok(v != 0)
    }
}

impl<P: ParamParse> ParamParse for Option<P> {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        if p.has_words() {
            P::parse(p).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl<P: ParamParse> ParamParse for Box<[P]> {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        // Most of the time we're parsing a list of single words, so use this
        // as a heuristic for pre-allocation
        let mut buf = if std::mem::size_of::<P>() <= std::mem::size_of::<u32>() {
            let len = p.params.len();
            Vec::with_capacity(len)
        } else {
            Vec::new()
        };

        while p.has_words() {
            let v = try!(P::parse(p));
            buf.push(v);
        }

        Ok(buf.into_boxed_slice())
    }
}

impl ParamParse for instruction::ExecutionMode {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        use desc::ExecutionMode as EMTag;
        use instruction::ExecutionMode::*;

        let mode : desc::ExecutionMode = try!(p.parse());
        let mode = match mode {
            EMTag::Invocations => {
                let n = try!(p.parse_word());
                Invocations(n)
            }
            EMTag::SpacingEqual => SpacingEqual,
            EMTag::SpacingFractionalEven => SpacingFractionalEven,
            EMTag::SpacingFractionalOdd => SpacingFractionalOdd,
            EMTag::VertexOrderCw => VertexOrderCw,
            EMTag::VertexOrderCcw => VertexOrderCcw,
            EMTag::PixelCenterInteger => PixelCenterInteger,
            EMTag::OriginUpperLeft => OriginUpperLeft,
            EMTag::OriginLowerLeft => OriginLowerLeft,
            EMTag::EarlyFragmentTests => EarlyFragmentTests,
            EMTag::PointMode => PointMode,
            EMTag::Xfb => Xfb,
            EMTag::DepthReplacing => DepthReplacing,
            EMTag::DepthGreater => DepthGreater,
            EMTag::DepthLess => DepthLess,
            EMTag::DepthUnchanged => DepthUnchanged,
            EMTag::LocalSize => {
                let x = try!(p.parse_word());
                let y = try!(p.parse_word());
                let z = try!(p.parse_word());
                LocalSize(x, y, z)
            }
            EMTag::LocalSizeHint => {
                let x = try!(p.parse_word());
                let y = try!(p.parse_word());
                let z = try!(p.parse_word());
                LocalSizeHint(x, y, z)
            }
            EMTag::InputPoints => InputPoints,
            EMTag::InputLines => InputLines,
            EMTag::InputLinesAdjacency => InputLinesAdjacency,
            EMTag::Triangles => Triangles,
            EMTag::InputTrianglesAdjacency => InputTrianglesAdjacency,
            EMTag::Quads => Quads,
            EMTag::IsoLines => IsoLines,
            EMTag::OutputVertices => {
                let n = try!(p.parse_word());
                OutputVertices(n)
            }
            EMTag::OutputPoints => OutputPoints,
            EMTag::OutputLineStrip => OutputLineStrip,
            EMTag::OutputTriangleStrip => OutputTriangleStrip,
            EMTag::VecTypeHint => {
                let ty = try!(p.parse_word());
                VecTypeHint(ty)
            }
            EMTag::ContractionOff => ContractionOff
        };

        Ok(mode)
    }
}

impl ParamParse for instruction::Decoration {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        use desc::Decoration as D;
        use instruction::Decoration::*;

        let decoration : desc::Decoration = try!(p.parse());
        let decoration = match decoration {
            D::RelaxedPrecision => RelaxedPrecision,
            D::SpecId => {
                let id = try!(p.parse_word());
                SpecId(id)
            }
            D::Block => Block,
            D::BufferBlock => BufferBlock,
            D::RowMajor => RowMajor,
            D::ColMajor => ColMajor,
            D::ArrayStride => {
                let n = try!(p.parse_word());
                ArrayStride(n)
            }
            D::MatrixStride  => {
                let n = try!(p.parse_word());
                MatrixStride(n)
            }
            D::GLSLShared => GLSLShared,
            D::GLSLPacked => GLSLPacked,
            D::CPacked => CPacked,
            D::BuiltIn => {
                let b = try!(p.parse());
                BuiltIn(b)
            }
            D::NoPerspective => NoPerspective,
            D::Flat => Flat,
            D::Patch => Patch,
            D::Centroid => Centroid,
            D::Sample => Sample,
            D::Invariant => Invariant,
            D::Restrict => Restrict,
            D::Aliased => Aliased,
            D::Volatile => Volatile,
            D::Constant => Constant,
            D::Coherent => Coherent,
            D::NonWritable => NonWritable,
            D::NonReadable => NonReadable,
            D::Uniform => Uniform,
            D::SaturatedConversion => SaturatedConversion,
            D::Stream => {
                let n = try!(p.parse_word());
                Stream(n)
            }
            D::Location => {
                let n = try!(p.parse_word());
                Location(n)
            }
            D::Component => {
                let n = try!(p.parse_word());
                Component(n)
            }
            D::Index => {
                let n = try!(p.parse_word());
                Index(n)
            }
            D::Binding => {
                let n = try!(p.parse_word());
                Binding(n)
            }
            D::DescriptorSet => {
                let n = try!(p.parse_word());
                DescriptorSet(n)
            }
            D::Offset => {
                let n = try!(p.parse_word());
                Offset(n)
            }
            D::XfbBuffer => {
                let n = try!(p.parse_word());
                XfbBuffer(n)
            }
            D::XfbStride => {
                let n = try!(p.parse_word());
                XfbStride(n)
            }
            D::FuncParamAttr => {
                let attr = try!(p.parse());
                FuncParamAttr(attr)
            }
            D::FPRoundingMode => {
                let attr = try!(p.parse());
                FPRoundingMode(attr)
            }
            D::FPFastMathMode => {
                let attr = try!(p.parse());
                FPFastMathMode(attr)
            }
            D::LinkageAttributes => {
                let name = try!(p.parse());
                let ty = try!(p.parse());
                LinkageAttributes(name, ty)
            }
            D::NoContraction => NoContraction,
            D::InputAttachmentIndex => {
                let idx = try!(p.parse_word());
                InputAttachmentIndex(idx)
            }
            D::Alignment => {
                let idx = try!(p.parse_word());
                Alignment(idx)
            }
        };

        Ok(decoration)
    }
}

impl ParamParse for ImageOperands {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        const OPERANDS : [desc::ImageOperands; 8] = [
            desc::ImgOpBias, desc::ImgOpLod, desc::ImgOpGrad,
            desc::ImgOpConstOffset, desc::ImgOpOffset,
            desc::ImgOpConstOffsets, desc::ImgOpSample,
            desc::ImgOpMinLod
        ];
        let mut operands = ImageOperands::new();

        if p.has_words() {
            let ops = try!(p.parse::<desc::ImageOperands>());

            for &o in OPERANDS.iter() {
                if ops.contains(o) {
                    let operand = try!(p.parse::<ValueId>());
                    operands.set(o, operand);
                }
            }
        }

        Ok(operands)
    }
}

macro_rules! impl_param_parse_word(
    (enum $($name:ident),+) => (
        $(impl ParamParse for ::desc::$name {
            fn parse(p: &mut InstructionParser) -> Result<Self> {
                let word = try!(p.parse_word());
                if let Some(val) = desc::$name::from(word) {
                    Ok(val)
                } else {
                    Err(ParseError::InvalidParamValue(word, stringify!($name)))
                }
            }
        })+
    );
    (bitset $($name:ident),+) => (
        $(impl ParamParse for ::desc::$name {
            fn parse(p: &mut InstructionParser) -> Result<Self> {
                let word = try!(p.parse_word());
                Ok(desc::$name::from(word))
            }
        })+
    );
);

impl_param_parse_word!(enum
    SrcLang,
    ExecutionModel,
    AddressingModel,
    MemoryModel,
    ExecutionMode,
    StorageClass,
    Dim,
    SamplerAddressingMode,
    SamplerFilterMode,
    ImageFormat,
    ImageChannelOrder,
    ImageChannelDataType,
    FPRoundingMode,
    LinkageType,
    AccessQualifier,
    FuncParamAttr,
    Decoration,
    BuiltIn,
    Scope,
    GroupOperation,
    KernelEnqueueFlags,
    Capability
);

impl_param_parse_word!(bitset
    ImageOperands,
    FPFastMathMode,
    SelectionControl,
    LoopControl,
    FunctionControl,
    MemoryOrdering,
    // MemoryAccess, // Handled below
    KernelProfilingInfo
);

impl ParamParse for desc::MemoryAccess {
    fn parse(p: &mut InstructionParser) -> Result<Self> {
        let word = if p.has_words() {
            try!(p.parse_word())
        } else {
            0
        };
        Ok(desc::MemoryAccess::from(word))
    }

}
