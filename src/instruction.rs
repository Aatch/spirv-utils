// Copyright 2016 James Miller
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use desc::{self, Id, TypeId, ValueId, ResultId};

include!(concat!(env!("OUT_DIR"), "/insts.rs"));

impl Instruction {
    #[inline(always)]
    pub fn defines_value(&self) -> Option<ValueId> {
        self.defines_value_inner().map(|r| r.to_value_id())
    }
}

#[derive(Clone, Debug)]
pub enum ExecutionMode {
    Invocations(u32),
    SpacingEqual,
    SpacingFractionalEven,
    SpacingFractionalOdd,
    VertexOrderCw,
    VertexOrderCcw,
    PixelCenterInteger,
    OriginUpperLeft,
    OriginLowerLeft,
    EarlyFragmentTests,
    PointMode,
    Xfb,
    DepthReplacing,
    DepthGreater,
    DepthLess,
    DepthUnchanged,
    LocalSize(u32, u32, u32),
    LocalSizeHint(u32, u32, u32),
    InputPoints,
    InputLines,
    InputLinesAdjacency,
    Triangles,
    InputTrianglesAdjacency,
    Quads,
    IsoLines,
    OutputVertices(u32),
    OutputPoints,
    OutputLineStrip,
    OutputTriangleStrip,
    VecTypeHint(u32),
    ContractionOff
}

impl ExecutionMode {
    pub fn to_desc(&self) -> desc::ExecutionMode {
        match self {
            &ExecutionMode::Invocations(_) => desc::ExecutionMode::Invocations,
            &ExecutionMode::SpacingEqual => desc::ExecutionMode::SpacingEqual,
            &ExecutionMode::SpacingFractionalEven => desc::ExecutionMode::SpacingFractionalEven,
            &ExecutionMode::SpacingFractionalOdd => desc::ExecutionMode::SpacingFractionalOdd,
            &ExecutionMode::VertexOrderCw => desc::ExecutionMode::VertexOrderCw,
            &ExecutionMode::VertexOrderCcw => desc::ExecutionMode::VertexOrderCcw,
            &ExecutionMode::PixelCenterInteger => desc::ExecutionMode::PixelCenterInteger,
            &ExecutionMode::OriginUpperLeft => desc::ExecutionMode::OriginUpperLeft,
            &ExecutionMode::OriginLowerLeft => desc::ExecutionMode::OriginLowerLeft,
            &ExecutionMode::EarlyFragmentTests => desc::ExecutionMode::EarlyFragmentTests,
            &ExecutionMode::PointMode => desc::ExecutionMode::PointMode,
            &ExecutionMode::Xfb => desc::ExecutionMode::Xfb,
            &ExecutionMode::DepthReplacing => desc::ExecutionMode::DepthReplacing,
            &ExecutionMode::DepthGreater => desc::ExecutionMode::DepthGreater,
            &ExecutionMode::DepthLess => desc::ExecutionMode::DepthLess,
            &ExecutionMode::DepthUnchanged => desc::ExecutionMode::DepthUnchanged,
            &ExecutionMode::LocalSize(_, _, _) => desc::ExecutionMode::LocalSize,
            &ExecutionMode::LocalSizeHint(_, _, _) => desc::ExecutionMode::LocalSizeHint,
            &ExecutionMode::InputPoints => desc::ExecutionMode::InputPoints,
            &ExecutionMode::InputLines => desc::ExecutionMode::InputLines,
            &ExecutionMode::InputLinesAdjacency => desc::ExecutionMode::InputLinesAdjacency,
            &ExecutionMode::Triangles => desc::ExecutionMode::Triangles,
            &ExecutionMode::InputTrianglesAdjacency => desc::ExecutionMode::InputTrianglesAdjacency,
            &ExecutionMode::Quads => desc::ExecutionMode::Quads,
            &ExecutionMode::IsoLines => desc::ExecutionMode::IsoLines,
            &ExecutionMode::OutputVertices(_) => desc::ExecutionMode::OutputVertices,
            &ExecutionMode::OutputPoints => desc::ExecutionMode::OutputPoints,
            &ExecutionMode::OutputLineStrip => desc::ExecutionMode::OutputLineStrip,
            &ExecutionMode::OutputTriangleStrip => desc::ExecutionMode::OutputTriangleStrip,
            &ExecutionMode::VecTypeHint(_) => desc::ExecutionMode::VecTypeHint,
            &ExecutionMode::ContractionOff => desc::ExecutionMode::ContractionOff
        }
    }
}

#[derive(Clone, Debug)]
pub enum Decoration {
    RelaxedPrecision,
    SpecId(u32),
    Block,
    BufferBlock,
    RowMajor,
    ColMajor,
    ArrayStride(u32),
    MatrixStride(u32),
    GLSLShared,
    GLSLPacked,
    CPacked,
    BuiltIn(desc::BuiltIn),
    NoPerspective,
    Flat,
    Patch,
    Centroid,
    Sample,
    Invariant,
    Restrict,
    Aliased,
    Volatile,
    Constant,
    Coherent,
    NonWritable,
    NonReadable,
    Uniform,
    SaturatedConversion,
    Stream(u32),
    Location(u32),
    Component(u32),
    Index(u32),
    Binding(u32),
    DescriptorSet(u32),
    Offset(u32),
    XfbBuffer(u32),
    XfbStride(u32),
    FuncParamAttr(desc::FuncParamAttr),
    FPRoundingMode(desc::FPRoundingMode),
    FPFastMathMode(desc::FPFastMathMode),
    LinkageAttributes(String, desc::LinkageType),
    NoContraction,
    InputAttachmentIndex(u32),
    Alignment(u32),
}

impl Decoration {
    pub fn to_desc(&self) -> desc::Decoration {
        match self {
            &Decoration::RelaxedPrecision => desc::Decoration::RelaxedPrecision,
            &Decoration::SpecId(_) => desc::Decoration::SpecId,
            &Decoration::Block => desc::Decoration::Block,
            &Decoration::BufferBlock => desc::Decoration::BufferBlock,
            &Decoration::RowMajor => desc::Decoration::RowMajor,
            &Decoration::ColMajor => desc::Decoration::ColMajor,
            &Decoration::ArrayStride(_) => desc::Decoration::ArrayStride,
            &Decoration::MatrixStride(_) => desc::Decoration::MatrixStride,
            &Decoration::GLSLShared => desc::Decoration::GLSLShared,
            &Decoration::GLSLPacked => desc::Decoration::GLSLPacked,
            &Decoration::CPacked => desc::Decoration::CPacked,
            &Decoration::BuiltIn(_) => desc::Decoration::BuiltIn,
            &Decoration::NoPerspective => desc::Decoration::NoPerspective,
            &Decoration::Flat => desc::Decoration::Flat,
            &Decoration::Patch => desc::Decoration::Patch,
            &Decoration::Centroid => desc::Decoration::Centroid,
            &Decoration::Sample => desc::Decoration::Sample,
            &Decoration::Invariant => desc::Decoration::Invariant,
            &Decoration::Restrict => desc::Decoration::Restrict,
            &Decoration::Aliased => desc::Decoration::Aliased,
            &Decoration::Volatile => desc::Decoration::Volatile,
            &Decoration::Constant => desc::Decoration::Constant,
            &Decoration::Coherent => desc::Decoration::Coherent,
            &Decoration::NonWritable => desc::Decoration::NonWritable,
            &Decoration::NonReadable => desc::Decoration::NonReadable,
            &Decoration::Uniform => desc::Decoration::Uniform,
            &Decoration::SaturatedConversion => desc::Decoration::SaturatedConversion,
            &Decoration::Stream(_) => desc::Decoration::Stream,
            &Decoration::Location(_) => desc::Decoration::Location,
            &Decoration::Component(_) => desc::Decoration::Component,
            &Decoration::Index(_) => desc::Decoration::Index,
            &Decoration::Binding(_) => desc::Decoration::Binding,
            &Decoration::DescriptorSet(_) => desc::Decoration::DescriptorSet,
            &Decoration::Offset(_) => desc::Decoration::Offset,
            &Decoration::XfbBuffer(_) => desc::Decoration::XfbBuffer,
            &Decoration::XfbStride(_) => desc::Decoration::XfbStride,
            &Decoration::FuncParamAttr(_) => desc::Decoration::FuncParamAttr,
            &Decoration::FPRoundingMode(_) => desc::Decoration::FPRoundingMode,
            &Decoration::FPFastMathMode(_) => desc::Decoration::FPFastMathMode,
            &Decoration::LinkageAttributes(_, _) => desc::Decoration::LinkageAttributes,
            &Decoration::NoContraction => desc::Decoration::NoContraction,
            &Decoration::InputAttachmentIndex(_) => desc::Decoration::InputAttachmentIndex,
            &Decoration::Alignment(_) => desc::Decoration::Alignment,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImageOperands {
    set: desc::ImageOperands,
    values: Vec<ValueId>
}

const ALL_OPERANDS : [desc::ImageOperands; 8] = [
    desc::ImgOpBias, desc::ImgOpLod, desc::ImgOpGrad,
    desc::ImgOpConstOffset, desc::ImgOpOffset,
    desc::ImgOpConstOffsets, desc::ImgOpSample,
    desc::ImgOpMinLod
];

impl ImageOperands {
    #[inline]
    pub fn new() -> ImageOperands {
        ImageOperands {
            set: desc::ImageOperands::empty(),
            values: Vec::new()
        }
    }

    pub fn get(&mut self, op: desc::ImageOperands) -> Option<ValueId> {
        assert!(op.count() == 1, "`op` must be single entry, got {:?}", op);

        if self.set.contains(op) {
            let idx = self.idx_of(op);
            Some(self.values[idx])
        } else {
            None
        }
    }

    pub fn set(&mut self, op: desc::ImageOperands, value: ValueId) {
        assert!(op.count() == 1, "`op` must be single entry, got {:?}", op);

        let idx = self.idx_of(op);
        assert!(idx <= self.values.len());

        // If it's already set, replace the existing value, otherwise
        // insert into the new position
        if self.set.contains(op) {
            self.values[idx] = value;
        } else {
            self.values.insert(idx, value);
            self.set.insert(op);
        }
    }

    fn idx_of(&self, op: desc::ImageOperands) -> usize {
        assert!(op.count() == 1, "`op` must be single entry, got {:?}", op);
        // Get the index where the value for this operand is/should be
        // this is effectively done by counting the set bits that are
        // lower than the operand we're checking
        let mut idx = 0;
        for &o in ALL_OPERANDS.iter() {
            // Stop once we reach this operand
            if o.bits() >= op.bits() {
                break;
            }
            // If the operand we're checking is in the set, bump the
            // index
            if self.set.contains(o) {
                idx += 1;
            }
        }

        idx
    }
}
