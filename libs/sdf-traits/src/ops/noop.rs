use sdf_vecs::{Vec3, VecType};

use crate::{Boxed, Operator, Spatial, VariableContainer};

#[derive(Clone)]
pub(crate) enum Variable {
    Const(VecType),
    Replaceable(Vec3)
}

#[derive(Clone)]
pub(crate) struct NoOp(pub(crate) Variable);

impl Spatial for NoOp {}

impl VariableContainer for NoOp {
    fn replace_variable(&mut self, var: &Vec3) {
        match &mut self.0 {
            Variable::Replaceable(r) => *r = *var,
            Variable::Const(_) => ()
        }
    }
}

impl Operator<VecType> for NoOp {
    fn operate(&self) -> VecType {
        match self.0 {
            Variable::Replaceable(r) => VecType::Vec3(r),
            Variable::Const(c) => c
        }
    }
}

impl NoOp {
    pub(crate) fn new_const(c: &VecType) -> Self {
        Self(Variable::Const(*c))
    }

    pub(crate) fn new_var() -> Self {
        Self(Variable::Replaceable(Vec3::default()))
    }
}

impl Boxed for NoOp {
    fn boxed(self) -> Box<dyn Spatial> {
        Box::new(self)
    }
}