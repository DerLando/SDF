use glam::Vec3;
use sdf_vecs::{VecType, ops::Abs as AbsTrait};
use crate::{Operator, Spatial};

impl_unary_op!(Abs, abs);

impl Operator for Abs {
    fn operate(&self, pos: &Vec3) -> VecType {
        self.0.operate(pos).abs()
    }
}