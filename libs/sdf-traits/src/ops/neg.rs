use sdf_vecs::{Vec3, VecType};

use crate::{Operator, Spatial, VariableContainer, Boxed};

impl_unary_op!(Neg);

impl Operator<VecType> for Neg {
    fn operate(&self) -> VecType {
        -self.0.operate()
    }
}