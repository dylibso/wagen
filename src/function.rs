use crate::*;

#[derive(Clone)]
pub struct Function<'a> {
    pub name: String,
    pub body: Builder<'a>,
    pub locals: Vec<ValType>,
    pub type_index: TypeIndex,
    pub index: FunctionIndex,
    pub export: Option<String>,
}

impl<'a> Function<'a> {
    pub fn push(&mut self, expr: impl Expr<'a>) -> &mut Self {
        self.body.push(expr);
        self
    }

    pub fn export(&mut self, name: impl Into<String>) -> &mut Self {
        self.export = Some(name.into());
        self
    }

    pub fn builder(&mut self) -> &mut Builder<'a> {
        &mut self.body
    }

    pub fn with_builder(&mut self, f: impl Fn(&mut Builder)) -> &mut Self {
        f(&mut self.body);
        self
    }

    pub fn index(&self) -> FunctionIndex {
        self.index
    }
}
