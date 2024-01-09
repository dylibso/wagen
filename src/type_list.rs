use crate::*;

#[derive(Clone, Debug)]
pub struct TypeList<T> {
    next: u32,
    pub items: std::collections::BTreeMap<u32, ValType>,
    _t: std::marker::PhantomData<T>,
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

handle!(LocalHandle);
pub type Local = Index<LocalHandle>;

impl Local {
    pub fn set<'a>(&self) -> impl Expr<'a> {
        Instr::LocalSet(self.0)
    }

    pub fn tee<'a>(&self) -> impl Expr<'a> {
        Instr::LocalTee(self.0)
    }
}

impl<'a> Expr<'a> for Local {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push(Instr::LocalGet(self.0));
    }
}

handle!(ParamHandle);
pub type Param = Index<ParamHandle>;

impl<'a> Expr<'a> for Param {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push(Instr::LocalGet(self.0));
    }
}
