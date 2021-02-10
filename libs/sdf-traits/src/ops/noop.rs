use sdf_vecs::{Vec1, Vec2, Vec3, Vec4, VecType};

use crate::{Operator, Spatial, VariableContainer};

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

impl From<f32> for NoOp {
    fn from(arg: f32) -> Self {
        NoOp::new_const(&VecType::Vec1(Vec1::new(arg)))
    }
}

impl From<Vec1> for NoOp {
    fn from(arg: Vec1) -> Self {
        NoOp::new_const(&VecType::Vec1(arg))
    }
}

impl From<Vec2> for NoOp {
    fn from(arg: Vec2) -> Self {
        NoOp::new_const(&VecType::Vec2(arg))
    }
}

impl From<Vec3> for NoOp {
    fn from(arg: Vec3) -> Self {
        NoOp::new_const(&VecType::Vec3(arg))
    }
}

impl From<Vec4> for NoOp {
    fn from(arg: Vec4) -> Self {
        NoOp::new_const(&VecType::Vec4(arg))
    }
}