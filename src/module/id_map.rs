use std::fmt;

use desc::{Id, TypeId, ValueId};
use module::{Type, Variable, Constant};

pub type IdMapBuilder<T> = RawIdMapBuilder<(Id, T)>;
pub type IdMap<T> = RawIdMap<(Id, T)>;

pub type TypeMapBuilder = RawIdMapBuilder<Type>;
pub type TypeMap = RawIdMap<Type>;

pub type ConstantMapBuilder = RawIdMapBuilder<Constant>;
pub type ConstantMap = RawIdMap<Constant>;

pub type VariableMapBuilder = RawIdMapBuilder<Variable>;
pub type VariableMap = RawIdMap<Variable>;

pub struct RawIdMapBuilder<V: GetId> {
    values: Vec<V>
}

#[derive(Clone)]
pub struct RawIdMap<V: GetId> {
    values: Vec<V>
}

impl<V: GetId> RawIdMapBuilder<V> {
    pub fn new() -> RawIdMapBuilder<V> {
        RawIdMapBuilder {
            values: Vec::new()
        }
    }

    pub fn insert(&mut self, val: V) {
        self.values.push(val);
    }

    pub fn get<'a>(&'a self, id: V::Id) -> Option<&'a V> {
        let id : Id = id.into();

        // We reverse because normally lookups at the build stage
        // are for elements more recently added
        self.values.iter().rev().find(|v| {
            let vid : Id = v.get_id().into();
            vid == id
        })
    }

    pub fn get_mut<'a>(&'a mut self, id: V::Id) -> Option<&'a mut V> {
        let id : Id = id.into();

        // We reverse because normally lookups at the build stage
        // are for elements more recently added
        self.values.iter_mut().rev().find(|v| {
            let vid : Id = v.get_id().into();
            vid == id
        })
    }

    pub fn into_id_map(mut self) -> RawIdMap<V> {
        self.values.sort_by(|a, b| {
            let a_id : Id = a.get_id().into();
            let b_id : Id = b.get_id().into();
            a_id.0.cmp(&b_id.0)
        });

        RawIdMap {
            values: self.values
        }
    }
}

impl<V: GetId> RawIdMap<V> {
    pub fn new() -> RawIdMap<V> {
        RawIdMap {
            values: Vec::new()
        }
    }

    pub fn insert(&mut self, val: V) {
        let id : Id = val.get_id().into();
        let idx = self.values.binary_search_by(|a| {
            let a_id : Id = a.get_id().into();
            a_id.0.cmp(&id.0)
        });

        match idx {
            Ok(idx) => {
                self.values[idx] = val;
            }
            Err(idx) => {
                self.values.insert(idx, val);
            }
        }
    }

    pub fn get<'a>(&'a self, id: V::Id) -> Option<&'a V> {
        self.get_index(id).map(|idx| {
            &self.values[idx]
        })
    }

    pub fn get_mut<'a>(&'a mut self, id: V::Id) -> Option<&'a mut V> {
        self.get_index(id).map(move |idx| {
            &mut self.values[idx]
        })
    }

    pub fn contains(&self, id: V::Id) -> bool {
        self.get_index(id).is_some()
    }

    fn get_index(&self, id: V::Id) -> Option<usize> {
        let id : Id = id.into();
        self.values.binary_search_by(|a| {
            let a_id : Id = a.get_id().into();
            a_id.0.cmp(&id.0)
        }).ok()
    }

    pub fn as_slice(&self) -> &[V] {
        &self.values[..]
    }
}

impl<V: GetId + fmt::Debug> fmt::Debug for RawIdMap<V> where V::Id: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(f.write_str("IdMap "));
        f.debug_map().entries(self.values.iter().map(|v| {
            (v.get_id(), v)
        })).finish()
    }
}

pub trait GetId {
    type Id: Into<Id>;

    fn get_id(&self) -> Self::Id;
}

impl GetId for Type {
    type Id = TypeId;

    fn get_id(&self) -> TypeId {
        self.id
    }
}

impl GetId for Constant {
    type Id = ValueId;

    fn get_id(&self) -> ValueId {
        self.id
    }
}

impl GetId for Variable {
    type Id = ValueId;

    fn get_id(&self) -> ValueId {
        self.id
    }
}

impl<I, T> GetId for (I, T) where I: Into<Id> + Copy {
    type Id = I;

    fn get_id(&self) -> I {
        self.0
    }
}
