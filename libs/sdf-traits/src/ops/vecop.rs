use sdf_vecs::{ComponentAccess, Vec2, Vec3, VecType};

use crate::{Operator, Spatial, VariableContainer, };



impl_binary_op!(Vec2Op, vec2);

impl Operator<VecType> for Vec2Op {
    fn operate(&self) -> VecType {
        Vec2::new(self.lhs.operate().x(), self.rhs.operate().x()).into()
    }
}