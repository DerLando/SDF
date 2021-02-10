use sdf_vecs::{Vec3, VecType};

use crate::{Operator, Spatial, VariableContainer, };

impl_unary_op!(Neg, neg);

impl Operator<VecType> for Neg {
    fn operate(&self) -> VecType {
        -self.0.operate()
    }
}