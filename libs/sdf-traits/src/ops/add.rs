use sdf_vecs::{Vec3, VecType, ops::add_high};

use crate::{Operator, Spatial, VariableContainer};

impl_binary_op!(Add);

impl Operator<VecType> for Add {
    fn operate(&self) -> VecType {
        add_high(&self.lhs.operate(), &self.rhs.operate())
    }
}
