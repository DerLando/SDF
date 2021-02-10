use sdf_vecs::{VecType, ops::Abs as AbsTrait};

use crate::{Operator, Spatial, VariableContainer};

use super::*;

impl_unary_op!(Abs);

impl Operator<VecType> for Abs {
    fn operate(&self) -> VecType {
        self.0.operate().abs()
    }
}