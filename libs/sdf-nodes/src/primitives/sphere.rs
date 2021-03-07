use std::rc::Rc;
use sdf_vecs::{VecType, Vec3, Vec2};
use crate::{constant::Constant, node::{UnaryNode, BinaryNode, BinaryNodeBuilder, Node}, variable::VariableType, ops::{UnaryOperator, BinaryOperator}};

pub(crate) fn sphere(radius: f32) -> Node {
    let r: Constant = radius.into();
    sub!(length!(VariableType::Variable), r)
}
