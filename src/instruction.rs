use desc::{self, Id, TypeId, ValueId, ResultId};

#[derive(Clone, Debug)]
pub enum Instruction {
    Nop,
    Undef(TypeId, ResultId),
    SourceContinued(String),
    Source(desc::SrcLang, u32, ValueId, Option<String>),
    SourceExtension(String),
    Name(Id, String),
    MemberName(TypeId, u32, String),
    String(ResultId, String),
    Line(ValueId, u32, u32),
    Extension(String),
    ExtInstImport(ResultId, String),
    ExtInst(TypeId, ResultId, ValueId, u32, Box<[Id]>),
    MemoryModel(desc::AddressingModel, desc::MemoryModel),
    EntryPoint(desc::ExecutionModel, ValueId, String, Box<[ValueId]>),
    ExecutionMode(ValueId, ExecutionMode),
    Capability(desc::Capability),
    TypeVoid(ResultId),
    TypeBool(ResultId),
    TypeInt(ResultId, u32, bool),
    TypeFloat(ResultId, u32),
    TypeVector(ResultId, TypeId, u32),
    TypeMatrix(ResultId, TypeId, u32),
    TypeImage(ResultId, TypeId, desc::Dim, u32,
              bool, bool, u32, desc::ImageFormat,
              Option<desc::AccessQualifier>),
    TypeSampler(ResultId),
    TypeSampledImage(ResultId, TypeId),
    TypeArray(ResultId, TypeId, ValueId),
    TypeRuntimeArray(ResultId, TypeId),
    TypeStruct(ResultId, Box<[TypeId]>),
    TypeOpaque(ResultId, String),
    TypePointer(ResultId, desc::StorageClass, TypeId),
    TypeFunction(ResultId, TypeId, Box<[TypeId]>),
    TypeEvent(ResultId),
    TypeDeviceEvent(ResultId),
    TypeReserveId(ResultId),
    TypeQueue(ResultId),
    TypePipe(ResultId),
    TypeForwardPointer(ResultId, desc::StorageClass),
    ConstantTrue(TypeId, ResultId),
    ConstantFalse(TypeId, ResultId),
    Constant(TypeId, ResultId, Box<[u32]>),
    ConstantComposite(TypeId, ResultId, Box<[ValueId]>),
    ConstantSampler(TypeId, ResultId, desc::SamplerAddressingMode, bool, desc::SamplerFilterMode),
    ConstantNull(TypeId, ResultId),
    SpecConstantTrue(TypeId, ResultId),
    SpecConstantFalse(TypeId, ResultId),
    SpecConstant(TypeId, ResultId, Box<[u32]>),
    SpecConstantComposite(TypeId, ResultId, Box<[ValueId]>),
    SpecConstantOp(TypeId, ResultId, u32, Box<[ValueId]>),
    Function(TypeId, ResultId, desc::FunctionControl, TypeId),
    FunctionParameter(TypeId, ResultId),
    FunctionEnd,
    FunctionCall(TypeId, ResultId, ValueId, Box<[ValueId]>),
    Variable(TypeId, ResultId, desc::StorageClass, ValueId),
    ImageTexelPointer(TypeId, ResultId, ValueId, ValueId, ValueId),
    Load(TypeId, ResultId, ValueId, desc::MemoryAccess),
    Store(ValueId, ValueId, desc::MemoryAccess),
    CopyMemory(ValueId, ValueId, desc::MemoryAccess),
    CopyMemorySized(ValueId, ValueId, ValueId, desc::MemoryAccess),
    AccessChain(TypeId, ResultId, ValueId, Box<[ValueId]>),
    InBoundsAccessChain(TypeId, ResultId, ValueId, Box<[ValueId]>),
    PtrAccessChain(TypeId, ResultId, ValueId, ValueId, Box<[ValueId]>),
    ArrayLength(TypeId, ResultId, ValueId, u32),
    GenericPtrMemSemantics(TypeId, ResultId, ValueId),
    InBoundsPtrAccessChain(TypeId, ResultId, ValueId, ValueId, Box<[ValueId]>),
    Decorate(Id, Decoration),
    MemberDecorate(TypeId, u32, Decoration),
    DecorationGroup(ResultId),
    GroupDecorate(ValueId, Box<[Id]>),
    GroupMemberDecorate(ValueId, Box<[(TypeId, u32)]>),
    VectorExtractDynamic(TypeId, ResultId, ValueId, ValueId),
    VectorInsertDynamic(TypeId, ResultId, ValueId, ValueId, ValueId),
    VectorShuffle(TypeId, ResultId, ValueId, ValueId, Box<[u32]>),
    CompositeConstruct(TypeId, ResultId, Box<[ValueId]>),
    CompositeExtract(TypeId, ResultId, ValueId, Box<[u32]>),
    CompositeInsert(TypeId, ResultId, ValueId, ValueId, Box<[u32]>),
    CopyObject(TypeId, ResultId, ValueId),
    Transpose(TypeId, ResultId, ValueId),
    SampledImage,
    ImageSampleImplicitLod,
    ImageSampleExplicitLod,
    ImageSampleDrefImplicitLod,
    ImageSampleDrefExplicitLod,
    ImageSampleProjImplicitLod,
    ImageSampleProjExplicitLod,
    ImageSampleProjDrefImplicitLod,
    ImageSampleProjDrefExplicitLod,
    ImageFetch,
    ImageGather,
    ImageDrefGather,
    ImageRead,
    ImageWrite,
    Image,
    ImageQueryFormat,
    ImageQueryOrder,
    ImageQuerySizeLod,
    ImageQuerySize,
    ImageQueryLod,
    ImageQueryLevels,
    ImageQuerySamples,
    ConvertFToU,
    ConvertFToS,
    ConvertSToF,
    ConvertUToF,
    UConvert,
    SConvert,
    FConvert,
    QuantizeToF16,
    ConvertPtrToU,
    SatConvertSToU,
    SatConvertUToS,
    ConvertUToPtr,
    PtrCastToGeneric,
    GenericCastToPtr,
    GenericCastToPtrExplicit,
    Bitcast,
    SNegate,
    FNegate,
    IAdd,
    FAdd,
    ISub,
    FSub,
    IMul,
    FMul,
    UDiv,
    SDiv,
    FDiv,
    UMod,
    SRem,
    SMod,
    FRem,
    FMod,
    VectorTimesScalar,
    MatrixTimesScalar,
    VectorTimesMatrix,
    MatrixTimesVector,
    MatrixTimesMatrix,
    OuterProduct,
    Dot,
    IAddCarry,
    ISubBorrow,
    UMulExtended,
    SMulExtended,
    Any,
    All,
    IsNan,
    IsInf,
    IsFinite,
    IsNormal,
    SignBitSet,
    LessOrGreater,
    Ordered,
    Unordered,
    LogicalEqual,
    LogicalNotEqual,
    LogicalOr,
    LogicalAnd,
    LogicalNot,
    Select,
    IEqual,
    INotEqual,
    UGreaterThan,
    SGreaterThan,
    UGreaterThanEqual,
    SGreaterThanEqual,
    ULessThan,
    SLessThan,
    ULessThanEqual,
    SLessThanEqual,
    FOrdEqual,
    FUnordEqual,
    FOrdNotEqual,
    FUnordNotEqual,
    FOrdLessThan,
    FUnordLessThan,
    FOrdGreaterThan,
    FUnordGreaterThan,
    FOrdLessThanEqual,
    FUnordLessThanEqual,
    FOrdGreaterThanEqual,
    FUnordGreaterThanEqual,
    ShiftRightLogical,
    ShiftRightArithmetic,
    ShiftLeftLogical,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Not,
    BitFieldInsert,
    BitFieldSExtract,
    BitFieldUExtract,
    BitReverse,
    BitCount,
    DPdx,
    DPdy,
    Fwidth,
    DPdxFine,
    DPdyFine,
    FwidthFine,
    DPdxCoarse,
    DPdyCoarse,
    FwidthCoarse,
    EmitVertex,
    EndPrimitive,
    EmitStreamVertex,
    EndStreamPrimitive,
    ControlBarrier,
    MemoryBarrier,
    AtomicLoad,
    AtomicStore,
    AtomicExchange,
    AtomicCompareExchange,
    AtomicCompareExchangeWeak,
    AtomicIIncrement,
    AtomicIDecrement,
    AtomicIAdd,
    AtomicISub,
    AtomicSMin,
    AtomicUMin,
    AtomicSMax,
    AtomicUMax,
    AtomicAnd,
    AtomicOr,
    AtomicXor,
    Phi,
    LoopMerge,
    SelectionMerge,
    Label,
    Branch,
    BranchConditional,
    Switch,
    Kill,
    Return,
    ReturnValue,
    Unreachable,
    LifetimeStart,
    LifetimeStop,
    GroupAsyncCopy,
    GroupWaitEvents,
    GroupAll,
    GroupAny,
    GroupBroadcast,
    GroupIAdd,
    GroupFAdd,
    GroupFMin,
    GroupUMin,
    GroupSMin,
    GroupFMax,
    GroupUMax,
    GroupSMax,
    ReadPipe,
    WritePipe,
    ReservedReadPipe,
    ReservedWritePipe,
    ReserveReadPipePackets,
    ReserveWritePipePackets,
    CommitReadPipe,
    CommitWritePipe,
    IsValidReserveId,
    GetNumPipePackets,
    GetMaxPipePackets,
    GroupReserveReadPipePackets,
    GroupReserveWritePipePackets,
    GroupCommitReadPipe,
    GroupCommitWritePipe,
    EnqueueMarker,
    EnqueueKernel,
    GetKernelNDrangeSubGroupCount,
    GetKernelNDrangeMaxSubGroupSize,
    GetKernelWorkGroupSize,
    GetKernelPreferredWorkGroupSizeMultiple,
    RetainEvent,
    ReleaseEvent,
    CreateUserEvent,
    IsValidEvent,
    SetUserEventStatus,
    CaptureEventProfilingInfo,
    GetDefaultQueue,
    BuildNDRange,
    ImageSparseSampleImplicitLod,
    ImageSparseSampleExplicitLod,
    ImageSparseSampleDrefImplicitLod,
    ImageSparseSampleDrefExplicitLod,
    ImageSparseSampleProjImplicitLod,
    ImageSparseSampleProjExplicitLod,
    ImageSparseSampleProjDrefImplicitLod,
    ImageSparseSampleProjDrefExplicitLod,
    ImageSparseFetch,
    ImageSparseGather,
    ImageSparseDrefGather,
    ImageSparseTexelsResident,
    NoLine,
    AtomicFlagTestAndSet,
    AtomicFlagClear,
    ImageSparseRead,

    Unknown(u16, Box<[u32]>)
}

impl Instruction {
    /*
    pub fn result_id(&self) -> Option<Id> {
        use self::Instruction::*;
        match *self {
            Undef(_, id) |
            String(id, _) |
            ExtInstImport(id, _) |
            ExtInst(_, id, _, _, _) |

            TypeVoid(id) |
            TypeBool(id) |
            TypeInt(id, _, _) |
            TypeFloat(id, _) |
            TypeVector(id, _, _) |
            TypeMatrix(id, _, _) |
            TypeImage(id, _, _, _, _, _, _, _, _) |
            TypeSampler(id) |
            TypeSampledImage(id, _) |
            TypeArray(id, _, _) |
            TypeRuntimeArray(id, _) |
            TypeStruct(id, _) |
            TypeOpaque(id, _) |
            TypePointer(id, _, _) |
            TypeFunction(id, _, _) |
            TypeEvent(id) |
            TypeDeviceEvent(id) |
            TypeReserveId(id) |
            TypeQueue(id) |
            TypePipe(id) |

            ConstantTrue(_, id) |
            ConstantFalse(_, id) |
            Constant(_, id, _) |
            ConstantComposite(_, id, _) |
            ConstantSampler(_, id, _, _, _) |
            ConstantNull(_, id) |
            SpecConstantTrue(_, id) |
            SpecConstantFalse(_, id) |
            SpecConstant(_, id, _) |
            SpecConstantComposite(_, id, _) |
            SpecConstantOp(_, id, _, _) |

            Function(_, id, _, _) |
            FunctionParameter(_, id) |
            FunctionCall(_, id, _, _) |

            Variable(_, id, _, _) |
            ImageTexelPointer(_, id, _, _, _) |

            Load(_, id, _, _) |
            AccessChain(_, id, _, _) |
            InBoundsAccessChain(_, id, _, _) |
            PtrAccessChain(_, id, _, _, _) |
            ArrayLength(_, id, _, _) |
            InBoundsPtrAccessChain(_, id, _, _, _) |
            DecorationGroup(id) |
            VectorExtractDynamic(_, id, _, _) |
            VectorInsertDynamic(_, id, _, _, _) |
            VectorShuffle(_, id, _, _, _) |
            CompositeConstruct(_, id, _) |
            CompositeExtract(_, id, _, _) |
            CompositeInsert(_, id, _, _, _)|
            CopyObject(_, id, _) |
            Transpose(_, id, _)
                => Some(id),
            _ => None,
        }
    }

    pub fn each_use_id<E, F: FnMut(Id) -> Result<(),E>>(&self, mut f: F) -> Result<(), E> {
        use self::Instruction::*;
        match *self {
            Undef(id, _) |
            Source(_, _, id, _) |
            Name(id, _) |
            MemberName(id, _, _) |
            Line(id, _, _) |
            ExecutionMode(id, _) |

            TypeVector(_, id, _) |
            TypeMatrix(_, id, _) |
            TypeImage(_, id, _, _, _, _, _, _, _) |
            TypeSampledImage(_, id) |
            TypeRuntimeArray(_, id) |
            TypePointer(_, _, id) |
            ConstantTrue(id, _) |
            ConstantFalse(id, _) |
            Constant(id, _, _) |
            ConstantSampler(id, _, _, _, _) |
            ConstantNull(id, _) |
            SpecConstantTrue(id, _) |
            SpecConstantFalse(id, _) |
            SpecConstant(id, _, _) |

            FunctionParameter(id, _) |
            Decorate(id, _) |
            MemberDecorate(id, _, _)
                => return f(id),

            TypeArray(_, id1, id2) |
            Function(id1, _, _, id2) |

            Variable(id1, _, _, id2) |

            Load(id1, _, id2, _) |
            Store(id1, id2, _) |
            CopyMemory(id1, id2, _) |
            ArrayLength(id1, _, id2, _) |
            GenericPtrMemSemantics(id1, _, id2) |
            CompositeExtract(id1, _, id2, _) |
            CopyObject(id1, _, id2) |
            Transpose(id1, _, id2)
                => {
                try!(f(id1));
                return f(id2);
            }

            CopyMemorySized(id1, id2, id3, _) |
            VectorExtractDynamic(id1, _, id2, id3) |
            VectorShuffle(id1, _, id2, id3, _) |
            CompositeInsert(id1, _, id2, id3, _)
                => {
                try!(f(id1));
                try!(f(id2));
                return f(id3);
            }

            ImageTexelPointer(id1, _, id2, id3, id4) |
            VectorInsertDynamic(id1, _, id2, id3, id4)
                => {
                try!(f(id1));
                try!(f(id2));
                try!(f(id3));
                return f(id4);
            }

            TypeStruct(_, ref ids) => {
                for &id in ids.iter() { try!(f(id)); }
            }

            EntryPoint(_, id, _, ref ids) |
            TypeFunction(_, id, ref ids) |
            ConstantComposite(id, _, ref ids) |
            SpecConstantComposite(id, _, ref ids) |
            SpecConstantOp(id, _, _, ref ids) |
            GroupDecorate(id, ref ids) |
            CompositeConstruct(id, _, ref ids)
                => {
                try!(f(id));
                for &id in ids.iter() { try!(f(id)); }
            }

            ExtInst(id1, _, id2, _, ref ids) |
            FunctionCall(id1, _, id2, ref ids) |
            AccessChain(id1, _, id2, ref ids) |
            InBoundsAccessChain(id1, _, id2, ref ids)
                => {
                try!(f(id1)); try!(f(id2));
                for &id in ids.iter() { try!(f(id)); }
            }
            PtrAccessChain(id1, _, id2, id3, ref ids) |
            InBoundsPtrAccessChain(id1, _, id2, id3, ref ids)
                => {
                try!(f(id1)); try!(f(id2)); try!(f(id3));
                for &id in ids.iter() { try!(f(id)); }
            }

            GroupMemberDecorate(id, ref pairs) => {
                try!(f(id));
                for &(id, _) in pairs.iter() {
                    try!(f(id));
                }
            }
            _ => ()
        }
        Ok(())
    }
    */
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
