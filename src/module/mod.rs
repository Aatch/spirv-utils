use std;
use std::io::Read;

use desc::{self, TypeId, ValueId, Id};
use instruction::{self, Instruction};
use parser::{self, Result};
use read::Reader;

#[derive(Clone, Debug)]
pub struct RawInstruction {
    pub opcode: u16,
    pub params: Vec<u32>
}

pub mod id_map;
pub mod decoration;

use self::id_map::{IdMap, TypeMap, VariableMap, ConstantMap};
use self::decoration::DecorationMap;

#[derive(Clone, Debug)]
pub struct Header {
    pub version: (u8,u8),
    pub generator_id: u32,
    pub id_bound: u32,
}

#[derive(Clone, Debug)]
pub struct Module {
    header: Header,
    capabilities: Vec<desc::Capability>,
    types: TypeMap,
    variables: VariableMap,
    constants: ConstantMap,
    names: IdMap<String>,
    member_names: IdMap<Vec<(u32, String)>>,
    decorations: DecorationMap,
    entry_points: Vec<EntryPoint>,
    instructions: Vec<Instruction>
}

impl Module {
    pub fn from_reader<R: Read>(rdr: &mut Reader<R>) -> Result<Module> {
        let header = try!(rdr.read_header());

        let mut capabilities = Vec::new();
        let mut types = id_map::TypeMapBuilder::new();
        let mut variables = id_map::VariableMapBuilder::new();
        let mut constants = id_map::ConstantMapBuilder::new();
        let mut names = id_map::IdMapBuilder::new();
        let mut member_names : id_map::IdMapBuilder<Vec<_>> = id_map::IdMapBuilder::new();
        let mut decorations = decoration::DecorationMapBuilder::new();
        let mut entry_points = Vec::new();
        let mut instructions = Vec::new();

        while let Some(raw_inst) = try!(rdr.read_instruction()) {
            let inst = try!(parser::parse_raw_instruction(raw_inst));

            match inst {
                Instruction::Capability(cap) => {
                    capabilities.push(cap);
                }
                Instruction::Name(id, ref n) => {
                    names.insert((id, n.clone()));
                }
                Instruction::MemberName(id, m, ref n) => {
                    let id : Id = id.into();
                    let n = n.clone();
                    if member_names.contains(id) {
                        if let Some(entry) = member_names.get_mut(id) {
                            entry.1.push((m, n));
                        }
                    } else {
                        member_names.insert((id, vec![(m, n)]));
                    }
                }
                Instruction::Decorate(id, ref dec) => {
                    decorations.insert_decoration(id, dec.clone());
                }
                Instruction::MemberDecorate(id, m, ref dec) => {
                    decorations.insert_member_decoration(id, m, dec.clone());
                }
                Instruction::EntryPoint(model, _, ref name, ref iface) => {
                    let ep = EntryPoint {
                        execution_model: model,
                        name: name.clone(),
                        interface: iface.clone()
                    };

                    entry_points.push(ep);
                }
                Instruction::TypeVoid(id) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::Void
                    };
                    types.insert(ty);
                }
                Instruction::TypeBool(id) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::Bool
                    };
                    types.insert(ty);
                }
                Instruction::TypeInt(id, w, true) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::Int(w)
                    };
                    types.insert(ty);
                }
                Instruction::TypeInt(id, w, false) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::UInt(w)
                    };
                    types.insert(ty);
                }
                Instruction::TypeFloat(id, w) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::Float(w)
                    };
                    types.insert(ty);
                }
                Instruction::TypeVector(id, el_id, n) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::Vector(el_id, n)
                    };
                    types.insert(ty);
                }
                Instruction::TypeMatrix(id, el_id, n) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::Matrix(el_id, n)
                    };
                    types.insert(ty);
                }
                Instruction::TypeImage(id, sty, dim, depth,
                                       arrayed, multisampled, sampled,
                                       fmt, acc) => {
                    let depth = match depth {
                        0 => ImageDepth::NoDepth,
                        1 => ImageDepth::Depth,
                        _ => ImageDepth::Unknown
                    };
                    let sampled = match sampled {
                        1 => ImageSampled::Sampled,
                        2 => ImageSampled::NotSampled,
                        _ => ImageSampled::Unknown
                    };

                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::Image {
                            sample_type: sty,
                            dim: dim,
                            depth: depth,
                            arrayed: arrayed,
                            multisampled: multisampled,
                            sampled: sampled,
                            image_format: fmt,
                            access_qualifier: acc
                        }
                    };
                    types.insert(ty);
                }
                Instruction::TypeSampler(id) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::Sampler
                    };
                    types.insert(ty);
                }
                Instruction::TypeSampledImage(id, img) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::SampledImage(img)
                    };
                    types.insert(ty);
                }
                Instruction::TypeArray(id, el_ty, len) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::Array(el_ty, len)
                    };
                    types.insert(ty);
                }
                Instruction::TypeRuntimeArray(id, el_ty) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::RuntimeArray(el_ty)
                    };
                    types.insert(ty);
                }
                Instruction::TypeStruct(id, ref flds) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::Struct(flds.clone())
                    };
                    types.insert(ty);
                }
                Instruction::TypePointer(id, sc, ptr) => {
                    let ty = Type {
                        id: id.to_type_id(),
                        ty: Ty::Pointer(sc, ptr)
                    };
                    types.insert(ty);
                }
                Instruction::ConstantNull(ty, id) => {
                    let c = Constant::null(id.to_value_id(), ty);
                    constants.insert(c);
                }
                Instruction::ConstantTrue(ty, id) => {
                    let c = Constant::bool(id.to_value_id(), ty, true);
                    constants.insert(c);
                }
                Instruction::ConstantFalse(ty, id) => {
                    let c = Constant::bool(id.to_value_id(), ty, false);
                    constants.insert(c);
                }
                Instruction::Constant(ty, id, ref vals) => {
                    let ty = types.get(ty).expect("Types should be defined before use");
                    let c = Constant::scalar(id.to_value_id(), ty, vals);
                    constants.insert(c);
                }
                Instruction::SpecConstantTrue(ty, id) => {
                    let spec_id = find_spec_id(&decorations, id.into());
                    assert!(spec_id.is_some(), "Specialisation constant has no spec id");

                    let mut c = Constant::bool(id.to_value_id(), ty, true);
                    c.spec_id = spec_id;
                    constants.insert(c);
                }

                Instruction::SpecConstantFalse(ty, id) => {
                    let spec_id = find_spec_id(&decorations, id.into());
                    assert!(spec_id.is_some(), "Specialisation constant has no spec id");

                    let mut c = Constant::bool(id.to_value_id(), ty, false);
                    c.spec_id = spec_id;
                    constants.insert(c);
                }

                Instruction::Variable(ty, id, sc, _) => {
                    let v = Variable {
                        id: id.to_value_id(),
                        ty: ty,
                        sc: sc
                    };

                    variables.insert(v);
                }
                _ => ()
            }

            instructions.push(inst);
        }

        return Ok(Module {
            header: header,
            capabilities: capabilities,
            types: types.into_id_map(),
            variables: variables.into_id_map(),
            constants: constants.into_id_map(),
            names: names.into_id_map(),
            member_names: member_names.into_id_map(),
            decorations: decorations.into_decoration_map(),
            entry_points: entry_points,
            instructions: instructions
        });

        fn find_spec_id(dec_map: &decoration::DecorationMapBuilder, id: Id) -> Option<u32> {
            let decorations = if let Some(d) = dec_map.get_decorations(id.into()) {
                d
            } else {
                return None;
            };

            for dec in decorations {
                if let &instruction::Decoration::SpecId(id) = dec {
                    return Some(id);
                }
            }

            None
        }
    }

    pub fn variables(&self) -> &[Variable] {
        self.variables.as_slice()
    }

    pub fn constants(&self) -> &[Constant] {
        self.constants.as_slice()
    }

    pub fn types(&self) -> &[Type] {
        self.types.as_slice()
    }

    pub fn entry_points(&self) -> &[EntryPoint] {
        &self.entry_points[..]
    }

    pub fn decorations<I: Into<Id>>(&self, id: I) -> Option<&[instruction::Decoration]> {
        self.decorations.get_decorations(id.into())
    }

    pub fn name<I: Into<Id>>(&self, id: I) -> Option<&str> {
        self.names.get(id.into()).map(|s| &s.1[..])
    }

    pub fn member_name(&self, id: TypeId, member: u32) -> Option<&str> {
        let id : Id = id.into();
        self.member_names.get(id.into()).and_then(|n| {
            n.1.iter().find(|e| e.0 == member)
        }).map(|s| &s.1[..])
    }

    pub fn ty(&self, id: TypeId) -> Option<&Type> {
        self.types.get(id)
    }

    pub fn variable(&self, id: ValueId) -> Option<&Variable> {
        self.variables.get(id)
    }

    pub fn ty_to_string(&self, ty: &Type) -> String {
        if let Some(name) = self.name(ty.id) {
            return name.to_owned();
        }
        match ty.ty {
            Ty::Void => "void".to_owned(),
            Ty::Bool => "bool".to_owned(),
            Ty::Int(w) => format!("i{}", w),
            Ty::UInt(w) => format!("u{}", w),
            Ty::Float(w) => format!("f{}", w),
            Ty::Vector(ty, n) => {
                let ty = self.ty(ty).expect("Missing element type");
                format!("vec<{} x {}>", self.ty_to_string(ty), n)
            }
            Ty::Matrix(ty, rn) => {
                let vty = self.ty(ty).expect("Missing column type");
                let (el_ty, cn) = if let Ty::Vector(ty, n) = vty.ty {
                    (ty, n)
                } else {
                    panic!("Matrix column must be vector type");
                };

                let ty = self.ty(el_ty).expect("Missing element type");
                format!("mat<{} x {}*{}>", self.ty_to_string(ty), rn, cn)
            }
            Ty::Image {
                sample_type,
                dim,
                depth,
                arrayed,
                multisampled,
                sampled,
                image_format,
                access_qualifier: _
            } => {
                let sample_ty = self.ty(sample_type).expect("Missing sample type");
                let sample_ty = self.ty_to_string(sample_ty);
                let dim = match dim {
                    desc::Dim::_1D => "1D",
                    desc::Dim::_2D => "2D",
                    desc::Dim::_3D => "3D",
                    desc::Dim::Cube => "cube",
                    desc::Dim::Rect => "rect",
                    desc::Dim::Buffer => "buffer",
                    desc::Dim::SubpassData => "subpass-data"
                };

                let depth = match depth {
                    ImageDepth::NoDepth => "",
                    ImageDepth::Depth => "depth ",
                    ImageDepth::Unknown => "maybe-depth "
                };

                let arrayed = if arrayed {
                    "array "
                } else {
                    ""
                };

                let multisampled = if multisampled {
                    "multisampled "
                } else {
                    ""
                };

                let sampled = match sampled {
                    ImageSampled::Unknown => "maybe-sampled",
                    ImageSampled::Sampled => "sampled",
                    ImageSampled::NotSampled => "not-sampled"
                };

                format!("image {} {} {}{}{}{} format:{:?}",
                        sample_ty, dim, depth, arrayed, multisampled, sampled, image_format)
            }
            Ty::SampledImage(img) => {
                let img = self.ty(img).expect("Missing image type");
                let dim = if let Ty::Image { dim, .. } = img.ty {
                    dim
                } else {
                    panic!("Sampled image not sampling image type")
                };

                let dim = match dim {
                    desc::Dim::_1D => "1D",
                    desc::Dim::_2D => "2D",
                    desc::Dim::_3D => "3D",
                    desc::Dim::Cube => "Cube",
                    desc::Dim::Rect => "Rect",
                    desc::Dim::Buffer => "Buffer",
                    desc::Dim::SubpassData => panic!("Invalid dimension for sampler")
                };

                format!("sampler{}({})", dim, img.id.0)
            }
            Ty::Array(ty, len) => {
                let len = self.constants.get(len).expect("Missing array length");
                let is_spec = len.spec_id.is_some();
                let len = match len.value {
                    ConstantValue::Int(i) => {
                        assert!(i >= 1, "Invalid array length {}", i);
                        i as u64
                    }
                    ConstantValue::UInt(i) => {
                        i
                    }
                    _ => panic!("Invalid array length type")
                };

                let ty = self.ty(ty).expect("Missing element type");
                if is_spec {
                    format!("[{}; spec({})]", self.ty_to_string(ty), len)
                } else {
                    format!("[{}; {}]", self.ty_to_string(ty), len)
                }

            }
            Ty::RuntimeArray(ty) => {
                let ty = self.ty(ty).expect("Missing element type");
                format!("[{}]", self.ty_to_string(ty))
            }
            Ty::Struct(ref tys) => {
                let mut s = "{".to_owned();
                let mut first = true;
                for &ty in &tys[..] {
                    if !first {
                        s.push_str(", ");
                    }
                    let ty = self.ty(ty).expect("Missing field type");
                    s.push_str(&self.ty_to_string(ty));
                    first = false;
                }

                s.push_str("}");
                s
            }
            Ty::Opaque(ref n) => {
                format!("opaque {}", n.to_owned())
            }
            Ty::Pointer(_, ty) => {
                let ty = self.ty(ty).expect("Missing pointee type");
                format!("*{}", self.ty_to_string(ty))
            }
            _ => "<TODO>".to_owned()
        }
    }

    pub fn tydef_to_string(&self, ty: &Type) -> String {
        match ty.ty {
            Ty::Struct(ref tys) => {
                let sty = ty.id;
                let mut s = "{".to_owned();
                let mut first = true;
                for (i, &ty) in tys.iter().enumerate() {
                    if !first {
                        s.push_str(", ");
                    }
                    let fname = self.member_name(sty, i as u32);
                    if let Some(fname) = fname {
                        s.push_str(fname);
                        s.push_str(": ");
                    }
                    let ty = self.ty(ty).expect("Missing field type");
                    s.push_str(&self.ty_to_string(ty));
                    first = false;
                }

                s.push_str("}");
                s
            }
            _ => self.ty_to_string(ty)
        }
    }
}

#[derive(Clone, Debug)]
pub struct EntryPoint {
    pub execution_model: desc::ExecutionModel,
    pub name: String,
    pub interface: Box<[ValueId]>
}

#[derive(Clone, Debug)]
pub struct Type {
    pub id: TypeId,
    pub ty: Ty
}

#[derive(Clone, Debug)]
pub enum Ty {
    Void,
    Bool,
    Int(u32),
    UInt(u32),
    Float(u32),
    Vector(TypeId, u32),
    Matrix(TypeId, u32),
    Image {
        sample_type: TypeId,
        dim: desc::Dim,
        depth: ImageDepth,
        arrayed: bool,
        multisampled: bool,
        sampled: ImageSampled,
        image_format: desc::ImageFormat,
        access_qualifier: Option<desc::AccessQualifier>
    },
    Sampler,
    SampledImage(TypeId),
    Array(TypeId, ValueId),
    RuntimeArray(TypeId),
    Struct(Box<[TypeId]>),
    Opaque(String),
    Pointer(desc::StorageClass, TypeId)
}

#[derive(Copy, Clone, Debug)]
pub enum ImageDepth {
    NoDepth,
    Depth,
    Unknown
}

#[derive(Copy, Clone, Debug)]
pub enum ImageSampled {
    Unknown,
    Sampled,
    NotSampled
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub id: ValueId,
    pub ty: TypeId,
    pub sc: desc::StorageClass
}

impl Variable {
    pub fn is_private(&self) -> bool {
        use desc::StorageClass::*;
        match self.sc {
            Private | Function => true,
            _ => false
        }
    }
}

#[derive(Clone, Debug)]
pub struct Constant {
    pub id: ValueId,
    pub ty: TypeId,
    pub spec_id: Option<u32>,
    pub value: ConstantValue
}

#[derive(Clone, Debug)]
pub enum ConstantValue {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    F32(f32),
    F64(f64),
    Composite(Box<[ValueId]>),
    Sampler(desc::SamplerAddressingMode, bool, desc::SamplerFilterMode),
    General(Box<[u32]>),
    Expr, // TODO: Constant expressions for specialisation constants
}

impl Constant {
    pub fn null(id: ValueId, ty: TypeId) -> Constant {
        Constant {
            id: id,
            ty: ty,
            spec_id: None,
            value: ConstantValue::Null
        }
    }

    pub fn bool(id: ValueId, ty: TypeId, val: bool) -> Constant {
        Constant {
            id: id,
            ty: ty,
            spec_id: None,
            value: ConstantValue::Bool(val)
        }
    }

    pub fn scalar(id: ValueId, ty: &Type, vals: &Box<[u32]>) -> Constant {
        let value = match ty.ty {
            Ty::UInt(w) if w <= 32 => {
                ConstantValue::UInt(vals[0] as u64)
            }
            Ty::UInt(w) if w <= 64 => {
                let mut val = vals[0] as u64;
                val |= (vals[1] as u64) << 32;
                ConstantValue::UInt(val)
            }
            Ty::Int(w) if w <= 32 => {
                let val = (vals[0] as i32) as i64;
                ConstantValue::Int(val)
            }
            Ty::Int(w) if w <= 64 => {
                let mut val = vals[0] as i64;
                val |= (vals[1] as i32 as i64) << 32;
                ConstantValue::Int(val)
            }
            Ty::Float(32) => {
                let val : f32 = unsafe { std::mem::transmute(vals[0]) };
                ConstantValue::F32(val)
            }
            Ty::Float(64) => {
                let mut val : f64 = 0.0;
                // Is this actually ok? Should be since the alignment requirement
                // is taken care of.
                unsafe {
                    let parts : *mut [u32; 2] = &mut val as *mut f64 as *mut _;
                    (*parts)[0] = vals[0];
                    (*parts)[1] = vals[1];
                }
                ConstantValue::F64(val)
            }
            _ => {
                ConstantValue::General(vals.clone())
            }
        };

        Constant {
            id: id,
            ty: ty.id,
            spec_id: None,
            value: value
        }
    }
}
