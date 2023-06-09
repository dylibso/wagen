use crate::*;

pub trait Expr<'a> {
    fn expr(self) -> Builder<'a>;
}

impl<'a, F: Fn() -> Builder<'a>> Expr<'a> for F {
    fn expr(self) -> Builder<'a> {
        self()
    }
}

impl<'a> Expr<'a> for Vec<Instr<'a>> {
    fn expr(self) -> Builder<'a> {
        self.into()
    }
}

impl<'a> Expr<'a> for &[Instr<'a>] {
    fn expr(self) -> Builder<'a> {
        self.to_vec().into()
    }
}

impl<'a, const N: usize> Expr<'a> for [Instr<'a>; N] {
    fn expr(self) -> Builder<'a> {
        self.to_vec().into()
    }
}

impl<'a> Expr<'a> for Instr<'a> {
    fn expr(self) -> Builder<'a> {
        Builder::new([self])
    }
}

impl<'a> Expr<'a> for Builder<'a> {
    fn expr(self) -> Builder<'a> {
        self
    }
}
