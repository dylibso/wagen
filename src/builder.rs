use crate::*;

#[derive(Default, Clone)]
pub struct Builder<'a> {
    pub instrs: Vec<Instr<'a>>,
}

impl<'a> From<Vec<Instr<'a>>> for Builder<'a> {
    fn from(instrs: Vec<Instr<'a>>) -> Self {
        Builder { instrs }
    }
}

impl<'a> Builder<'a> {
    pub fn new(init: impl IntoIterator<Item = Instr<'a>>) -> Self {
        Builder {
            instrs: init.into_iter().collect(),
        }
    }

    pub fn push(&mut self, x: impl Expr<'a>) -> &mut Self {
        self.instrs.extend(x.expr().instrs);
        self
    }

    pub fn local_incr(&mut self, index: u32, ty: ValType, tee: bool) -> &mut Self {
        self.push(Instr::LocalGet(index));
        match ty {
            ValType::I64 => {
                self.push([Instr::I64Const(1), Instr::I64Add]);
            }
            ValType::I32 => {
                self.push([Instr::I32Const(1), Instr::I32Add]);
            }
            ValType::F64 => {
                self.push([Instr::F64Const(1.0), Instr::F64Add]);
            }
            ValType::F32 => {
                self.push([Instr::F32Const(1.0), Instr::F32Add]);
            }
            x => panic!("Invalid type in `local_incr`: {x:?}"),
        }

        if tee {
            self.push(Instr::LocalTee(index));
        } else {
            self.push(Instr::LocalSet(index));
        }
        self
    }

    pub fn block<F: Fn(&mut Self)>(&mut self, bt: BlockType, expr: F) -> &mut Self {
        self.push(Instr::Block(bt));
        expr(self);
        self.push(Instr::End)
    }

    pub fn r#loop<F: Fn(&mut Self)>(&mut self, bt: BlockType, expr: F) -> &mut Self {
        self.push(Instr::Loop(bt));
        expr(self);
        self.push(Instr::End)
    }

    pub fn if_then<F: Fn(&mut Self)>(
        &mut self,
        bt: BlockType,
        cond: impl Expr<'a>,
        expr: F,
    ) -> &mut Self {
        self.push(cond);
        self.push(Instr::If(bt));
        expr(self);
        self.push(Instr::End)
    }

    pub fn if_then_else<F: Fn(&mut Self), G: Fn(&mut Self)>(
        &mut self,
        bt: BlockType,
        cond: impl Expr<'a>,
        expr: F,
        else_: G,
    ) -> &mut Self {
        self.push(cond);
        self.push(Instr::If(bt));
        expr(self);
        self.push(Instr::Else);
        else_(self);
        self.push(Instr::End)
    }
}
