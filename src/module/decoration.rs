use desc::{TypeId, Id};
use instruction::Decoration;
use module::id_map::{IdMap, IdMapBuilder};

type MemberDecoration = (u32, Vec<Decoration>);

#[derive(Clone, Debug)]
pub struct DecorationMap {
    decorations: IdMap<Vec<Decoration>>,
    member_decorations: IdMap<Vec<MemberDecoration>>
}

pub struct DecorationMapBuilder {
    group_decoration: IdMap<Vec<Decoration>>,
    decorations: IdMapBuilder<Vec<Decoration>>,
    member_decorations: IdMapBuilder<Vec<MemberDecoration>>
}

impl DecorationMap {
    pub fn get_decorations<'a>(&'a self, id: Id) -> Option<&'a [Decoration]> {
        self.decorations.get(id).map(|d| &d.1[..])
    }
    pub fn get_member_decorations<'a>(&'a self, id: Id, member: u32) -> Option<&'a [Decoration]> {
        self.member_decorations.get(id).and_then(|v| {
            v.1.iter().find(|e| e.0 == member)
        }).map(|d| &d.1[..])
    }
}

impl DecorationMapBuilder {
    pub fn new() -> DecorationMapBuilder {
        DecorationMapBuilder {
            group_decoration: IdMap::new(),
            decorations: IdMapBuilder::new(),
            member_decorations: IdMapBuilder::new(),
        }
    }

    pub fn get_decorations<'a>(&'a self, id: Id) -> Option<&'a [Decoration]> {
        self.decorations.get(id).map(|d| &d.1[..])
    }
    pub fn get_member_decorations<'a>(&'a self, id: Id, member: u32) -> Option<&'a [Decoration]> {
        self.member_decorations.get(id).and_then(|v| {
            v.1.iter().find(|e| e.0 == member)
        }).map(|d| &d.1[..])
    }

    pub fn insert_decoration(&mut self, id: Id, decoration: Decoration) {
        if let Some(entry) = self.group_decoration.get_mut(id) {
            entry.1.push(decoration);
            return;
        }
        if let Some(entry) = self.decorations.get_mut(id) {
            entry.1.push(decoration);
            return;
        }
        self.decorations.insert((id, vec![decoration]));
    }
    pub fn insert_member_decoration(&mut self, id: TypeId, member: u32, decoration: Decoration) {
        let id : Id = id.into();
        if let Some(entry) = self.member_decorations.get_mut(id) {
            if let Some(mentry) = entry.1.iter_mut().find(|e| e.0 == member) {
                mentry.1.push(decoration);
                return;
            }
            entry.1.push((member, vec![decoration]));
            return;
        }
        let member_decoration = (member, vec![decoration]);
        self.member_decorations.insert((id, vec![member_decoration]));
    }
    pub fn insert_group(&mut self, id: Id) {
        self.group_decoration.insert((id, Vec::new()));
    }

    pub fn into_decoration_map(self) -> DecorationMap {
        DecorationMap {
            decorations: self.decorations.into_id_map(),
            member_decorations: self.member_decorations.into_id_map(),
        }
    }
}
