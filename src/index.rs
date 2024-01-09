use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Index<T>(pub u32, std::marker::PhantomData<T>);

#[macro_export]
macro_rules! handle {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
        pub struct $name;
    };
}

impl<T> From<u32> for Index<T> {
    fn from(value: u32) -> Self {
        Index(value, Default::default())
    }
}

impl<T> From<Index<T>> for u32 {
    fn from(value: Index<T>) -> Self {
        value.0
    }
}

impl<T> Index<T> {
    pub fn index(&self) -> u32 {
        self.0
    }
}

handle!(FunctionHandle);
pub type FunctionIndex = Index<FunctionHandle>;

impl<'a> Expr<'a> for FunctionIndex {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push(Instr::Call(self.0));
    }
}
