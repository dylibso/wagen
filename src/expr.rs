use crate::*;

pub trait Expr<'a> {
    fn expr(self, builder: &mut Builder<'a>);
}

impl<'a, F: FnOnce(&mut Builder<'a>)> Expr<'a> for F {
    fn expr(self, builder: &mut Builder<'a>) {
        self(builder)
    }
}

impl<'a> Expr<'a> for Vec<Instr<'a>> {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.extend(self);
    }
}

impl<'a, const N: usize> Expr<'a> for [Instr<'a>; N] {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.extend(self);
    }
}

impl<'a> Expr<'a> for &[Instr<'a>] {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.extend(self.to_vec());
    }
}

impl<'a> Expr<'a> for Instr<'a> {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push([self]);
    }
}

impl<'a> Expr<'a> for Builder<'a> {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.extend(self.instrs);
    }
}

impl<'a> Expr<'a> for bool {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push(Instr::I32Const(self as i32));
    }
}

impl<'a> Expr<'a> for i32 {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push(Instr::I32Const(self));
    }
}

impl<'a> Expr<'a> for i64 {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push(Instr::I64Const(self));
    }
}

impl<'a> Expr<'a> for f32 {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push(Instr::F32Const(self));
    }
}

impl<'a> Expr<'a> for f64 {
    fn expr(self, builder: &mut Builder<'a>) {
        builder.push(Instr::F64Const(self));
    }
}
