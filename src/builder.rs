use crate::*;

#[derive(Default, Debug, Clone)]
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
        x.expr(self);
        self
    }

    pub fn extend(&mut self, x: impl IntoIterator<Item = Instr<'a>>) -> &mut Self {
        self.instrs.extend(x);
        self
    }

    pub fn local_incr(&mut self, local: Local, ty: ValType) -> &mut Self {
        self.push(Instr::LocalGet(local.0));
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
        self.push(Instr::LocalSet(local.0));
        self
    }

    pub fn block<F: Expr<'a>>(&mut self, bt: BlockType, expr: F) -> &mut Self {
        self.push(Instr::Block(bt));
        self.push(expr);
        self.push(Instr::End)
    }

    pub fn loop_<F: Expr<'a>>(&mut self, bt: BlockType, expr: F) -> &mut Self {
        self.push(Instr::Loop(bt));
        self.push(expr);
        self.push(Instr::End)
    }

    pub fn loop_while<C: Expr<'a>, F: Expr<'a>>(
        &mut self,
        bt: BlockType,
        cond: C,
        expr: F,
    ) -> &mut Self {
        self.push(Instr::Loop(bt));
        self.push(cond);
        self.push(expr);
        self.push(Instr::BrIf(0));
        self.push(Instr::End)
    }

    pub fn if_then<F: Expr<'a>>(
        &mut self,
        bt: BlockType,
        cond: impl Expr<'a>,
        expr: F,
    ) -> &mut Self {
        self.push(cond);
        self.push(Instr::If(bt));
        self.push(expr);
        self.push(Instr::End)
    }

    pub fn if_then_else<F: Expr<'a>, G: Expr<'a>>(
        &mut self,
        bt: BlockType,
        cond: impl Expr<'a>,
        expr: F,
        else_: G,
    ) -> &mut Self {
        self.push(cond);
        self.push(Instr::If(bt));
        self.push(expr);
        self.push(Instr::Else);
        self.push(else_);
        self.push(Instr::End)
    }

    pub fn return_(&mut self) {
        self.push(Instr::Return);
    }
}
