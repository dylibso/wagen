use crate::*;

#[derive(Clone, Debug)]
pub struct TypeList<T> {
    next: u32,
    _t: std::marker::PhantomData<T>,
    pub items: std::collections::BTreeMap<u32, ValType>,
}

impl<T: From<u32>, U: IntoIterator<Item = ValType>> From<U> for TypeList<T> {
    fn from(value: U) -> Self {
        let mut dest = Self::new();
        for x in value.into_iter() {
            dest.push(x);
        }
        dest
    }
}

impl<T: From<u32>> Default for TypeList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: From<u32>> TypeList<T> {
    pub fn new() -> Self {
        Self {
            next: 0,
            _t: Default::default(),
            items: Default::default(),
        }
    }

    pub fn push(&mut self, ty: ValType) -> T {
        let n = self.next;
        self.items.insert(n, ty);
        self.next += 1;
        T::from(n)
    }

    pub fn ty(&self, local: Local) -> Option<ValType> {
        self.items.get(&local.0).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Local(pub u32);

impl Local {
    pub fn index(&self) -> u32 {
        self.0
    }
}

impl From<Local> for u32 {
    fn from(value: Local) -> Self {
        value.0
    }
}

impl From<u32> for Local {
    fn from(value: u32) -> Self {
        Local(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Param(pub u32);

impl Param {
    pub fn index(&self) -> u32 {
        self.0
    }
}

impl From<Param> for u32 {
    fn from(value: Param) -> Self {
        value.0
    }
}

impl From<u32> for Param {
    fn from(value: u32) -> Self {
        Param(value)
    }
}
