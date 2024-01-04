use crate::*;

pub trait Expr<'a> {
    fn expr(self, builder: &mut Builder<'a>);
}

impl<'a, F: Fn() -> Builder<'a>> Expr<'a> for F {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.extend(self().instrs);
    }
}

impl<'a> Expr<'a> for Vec<Instr<'a>> {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.extend(self);
    }
}

impl<'a> Expr<'a> for &[Instr<'a>] {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.extend(self.to_vec());
    }
}

impl<'a, const N: usize> Expr<'a> for [Instr<'a>; N] {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.extend(self.to_vec());
    }
}

impl<'a> Expr<'a> for Instr<'a> {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push(self);
    }
}

impl<'a> Expr<'a> for Builder<'a> {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.extend(self.instrs);
    }
}
