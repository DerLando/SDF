use sdf_vecs::{Vec3, VecType, ops::add_high};

use crate::{Operator, Spatial, VariableContainer, };

impl_binary_op!(Add, add);

impl Operator for Add {
    fn operate(&self, pos: &Vec3) -> VecType {
        add_high(&self.lhs.operate(pos), &self.rhs.operate(pos))
    }
}
