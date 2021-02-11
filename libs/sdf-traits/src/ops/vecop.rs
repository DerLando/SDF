use sdf_vecs::{ComponentAccess, Vec2, Vec3, VecType};

use crate::{Operator, Spatial};



impl_binary_op!(Vec2Op, vec2);

impl Operator for Vec2Op {
    fn operate(&self, pos: &Vec3) -> VecType {
        Vec2::new(self.lhs.operate(pos).x(), self.rhs.operate(pos).x()).into()
    }
}