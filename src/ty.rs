// Copyright 2016 James Miller
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::mem;

use typed_arena::Arena;

use instruction::{Decoration, Instruction};
use desc::{self, TypeId, ValueId};

type TypeIdMap<T> = HashMap<TypeId, T>;

pub struct TypeContext {
    type_arena: Arena<Type<'static>>,
    type_map: RefCell<TypeIdMap<&'static Type<'static>>>,
}

impl TypeContext {
    pub fn new() -> TypeContext {
        TypeContext {
            type_arena: Arena::new(),
            type_map: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_ty<'a>(&'a self, id: TypeId) -> Option<Ty<'a>> {
        let map = self.type_map.borrow();
        map.get(&id).cloned()
    }

    pub fn mk_ty<'a>(&'a self, ty: Type<'a>) -> Ty<'a> {
        // Don't add forward pointers to the map, they're only there to break cycles
        if let Type::ForwardPointer { .. } = ty {
            let ty : Type<'static> = unsafe { mem::transmute(ty) };
            let ty = self.type_arena.alloc(ty);
            let ty : &'static Type<'static> = unsafe { mem::transmute(ty) };
            return ty;
        }

        let id = ty.id();
        let mut map = self.type_map.borrow_mut();
        *map.entry(id).or_insert_with(|| {
            // Cast to static to "trick" the borrow/dtor checker. Otherwise
            // we have to use a &Arena and be supplied it from the top, which
            // makes for terrible API. This is safe because all the data is stored
            // in the arena, and we never hand out references with lifetimes
            // greater than the context
            let ty : Type<'static> = unsafe { mem::transmute(ty) };
            let ty = self.type_arena.alloc(ty);
            let ty : &'static Type<'static> = unsafe { mem::transmute(ty) };
            ty
        })
    }

    pub fn mk_ty_from_inst<'a>(&'a self, inst: &Instruction) -> Ty<'a> {
        let ty = match *inst {
            Instruction::TypeVoid {
                result_type,
            } => {
                Type::Void {
                    id: result_type,
                }
            },
            Instruction::TypeBool {
                result_type,
            } => {
                Type::Bool {
                    id: result_type,
                }
            },
            Instruction::TypeInt {
                result_type,
                width,
                signed,
            } => {
                Type::Int {
                    id: result_type,
                    width: width,
                    signed: signed,
                }
            },
            Instruction::TypeFloat {
                result_type,
                width,
            } => {
                Type::Float {
                    id: result_type,
                    width: width,
                }
            },
            Instruction::TypeVector {
                result_type,
                type_id,
                len,
            } => {
                let element = self.get_ty(type_id).unwrap();
                Type::Vector {
                    id: result_type,
                    element: element,
                    len: len,
                }
            },
            Instruction::TypeMatrix {
                result_type,
                type_id,
                cols,
            } => {
                let column = self.get_ty(type_id).unwrap();
                Type::Matrix {
                    id: result_type,
                    column: column,
                    cols: cols
                }
            },
            Instruction::TypeImage {
                result_type,
                type_id,
                dim,
                depth,
                arrayed,
                multisampled,
                sampled,
                image_format,
                access_qualifier,
            } => {
                let image = self.get_ty(type_id).unwrap();
                Type::Image {
                    id: result_type,
                    image: image,
                    dim: dim,
                    depth: depth,
                    arrayed: arrayed,
                    multisampled: multisampled,
                    sampled: sampled,
                    image_format: image_format,
                    access_qualifier: access_qualifier,
                }
            },
            Instruction::TypeSampler {
                result_type,
            } => {
                Type::Sampler {
                    id: result_type
                }
            },
            Instruction::TypeSampledImage {
                result_type,
                image,
            } => {
                let image = self.get_ty(image).unwrap();
                Type::SampledImage {
                    id: result_type,
                    image: image
                }
            },
            Instruction::TypeArray {
                result_type,
                element,
                len,
            } => {
                let element = self.get_ty(element).unwrap();
                Type::Array {
                    id: result_type,
                    element: element,
                    len: len,
                }
            },
            Instruction::TypeRuntimeArray {
                result_type,
                element,
            } => {
                let element = self.get_ty(element).unwrap();
                Type::RuntimeArray {
                    id: result_type,
                    element: element,
                }
            },
            Instruction::TypeStruct {
                result_type,
                ref fields,
            } => {
                let fld_tys = fields.iter().map(|&ty| {
                    self.get_ty(ty).unwrap()
                }).collect::<Vec<_>>().into_boxed_slice();
                Type::Struct {
                    id: result_type,
                    fields: fld_tys
                }
            },
            Instruction::TypeOpaque {
                result_type,
                ref name,
            } => {
                Type::Opaque {
                    id: result_type,
                    name: name.clone()
                }
            },
            Instruction::TypePointer {
                result_type,
                storage_class,
                pointee,
            } => {
                let pointee = self.get_ty(pointee).unwrap();
                Type::Pointer {
                    id: result_type,
                    storage_class: storage_class,
                    pointee: pointee
                }
            },
            Instruction::TypeFunction {
                result_type,
                return_ty,
                ref params,
            } => {
                let return_ty = self.get_ty(return_ty).unwrap();
                let params = params.iter().map(|&ty| {
                    self.get_ty(ty).unwrap()
                }).collect::<Vec<_>>().into_boxed_slice();
                Type::Function {
                    id: result_type,
                    return_ty: return_ty,
                    params: params
                }
            },
            Instruction::TypeEvent {
                result_type,
            } => {
                Type::Event {
                    id: result_type
                }
            },
            Instruction::TypeDeviceEvent {
                result_type,
            } => {
                Type::DeviceEvent {
                    id: result_type
                }
            },
            Instruction::TypeReserveId {
                result_type,
            } => {
                Type::ReserveId {
                    id: result_type
                }
            },
            Instruction::TypeQueue {
                result_type,
            } => {
                Type::Queue {
                    id: result_type
                }
            },
            Instruction::TypePipe {
                result_type,
            } => {
                Type::Pipe {
                    id: result_type
                }
            },
            Instruction::TypeForwardPointer {
                type_id,
                storage_class
            } => {
                Type::ForwardPointer {
                    id: type_id,
                    storage_class: storage_class
                }
            }
            _ => panic!("Non-type instruction `{:?}` given", inst)
        };

        self.mk_ty(ty)
    }

    /**
     * Returns the "real" type for the given type, basically just converts
     * ForwardPointers into regular pointers.
     *
     * Returns the actual Pointer type for a ForwardPointer, if it exists,
     * or returns the input.
     */
    pub fn real_type<'a>(&'a self, t: Ty<'a>) -> Option<Ty<'a>> {
        match *t {
            Type::ForwardPointer { id, .. } => self.get_ty(id),
            _ => Some(t)
        }
    }

    pub fn types<'a>(&'a self) -> Types<'a> {
        let map = self.type_map.borrow();
        let ids : Vec<_> = map.keys().cloned().collect();
        Types {
            ids: ids.into_iter(),
            ctxt: self
        }
    }
}

pub struct Types<'a> {
    ids: std::vec::IntoIter<TypeId>,
    ctxt: &'a TypeContext
}

impl<'a> Iterator for Types<'a> {
    type Item = Ty<'a>;

    fn next(&mut self) -> Option<Ty<'a>> {
        self.ids.next().and_then(|id| self.ctxt.get_ty(id))
    }
}

pub type Ty<'a> = &'a Type<'a>;

#[derive(Debug, Clone)]
pub enum Type<'a> {
    Void {
        id: TypeId,
    },
    Bool {
        id: TypeId,
    },
    Int {
        id: TypeId,
        width: u32,
        signed: bool,
    },
    Float {
        id: TypeId,
        width: u32,
    },
    Vector {
        id: TypeId,
        element: Ty<'a>,
        len: u32,
    },
    Matrix {
        id: TypeId,
        column: Ty<'a>,
        cols: u32,
    },
    Image {
        id: TypeId,
        image: Ty<'a>,
        dim: desc::Dim,
        depth: u32,
        arrayed: bool,
        multisampled: bool,
        sampled: u32,
        image_format: desc::ImageFormat,
        access_qualifier: Option<desc::AccessQualifier>,
    },
    Sampler {
        id: TypeId,
    },
    SampledImage {
        id: TypeId,
        image: Ty<'a>,
    },
    Array {
        id: TypeId,
        element: Ty<'a>,
        len: ValueId,
    },
    RuntimeArray {
        id: TypeId,
        element: Ty<'a>,
    },
    Struct {
        id: TypeId,
        fields: Box<[Ty<'a>]>,
    },
    Opaque {
        id: TypeId,
        name: String,
    },
    Pointer {
        id: TypeId,
        storage_class: desc::StorageClass,
        pointee: Ty<'a>,
    },
    Function {
        id: TypeId,
        return_ty: Ty<'a>,
        params: Box<[Ty<'a>]>,
    },
    Event {
        id: TypeId,
    },
    DeviceEvent {
        id: TypeId,
    },
    ReserveId {
        id: TypeId,
    },
    Queue {
        id: TypeId,
    },
    Pipe {
        id: TypeId,
    },
    ForwardPointer {
        id: TypeId,
        storage_class: desc::StorageClass
    },
}

impl<'a> Type<'a> {

    pub fn id(&self) -> TypeId {
        match *self {
            Type::Void           { id }     |
            Type::Bool           { id }     |
            Type::Int            { id, .. } |
            Type::Float          { id, .. } |
            Type::Vector         { id, .. } |
            Type::Matrix         { id, .. } |
            Type::Image          { id, .. } |
            Type::Sampler        { id }     |
            Type::SampledImage   { id, .. } |
            Type::Array          { id, .. } |
            Type::RuntimeArray   { id, .. } |
            Type::Struct         { id, .. } |
            Type::Opaque         { id, .. } |
            Type::Pointer        { id, .. } |
            Type::Function       { id, .. } |
            Type::Event          { id }     |
            Type::DeviceEvent    { id }     |
            Type::ReserveId      { id }     |
            Type::Queue          { id }     |
            Type::Pipe           { id }     |
            Type::ForwardPointer { id, .. } => { id }
        }
    }

    pub fn is_struct(&self) -> bool {
        match *self {
            Type::Struct { .. } => true,
            _ => false
        }
    }

    pub fn is_pointer(&self) -> bool {
        match *self {
            Type::Pointer { .. } |
            Type::ForwardPointer { .. } => true,
            _ => false
        }
    }

    pub fn pointee_ty(&self) -> Option<Ty<'a>> {
        match *self {
            Type::Pointer      { pointee, .. } => Some(pointee),
            _ => None
        }
    }
}
