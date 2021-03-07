use std::rc::Rc;
use sdf_vecs::{VecType, Vec3, Vec2};
use crate::{constant::Constant, node::{UnaryNode, BinaryNode, BinaryNodeBuilder, Node}, variable::VariableType, ops::{UnaryOperator, BinaryOperator}};

pub(crate) fn torus(inner_radius: f32, outer_radius: f32) -> Node {
    // q = vec2(length(p.xz)-t.x,p.y)
    // distance = length(q)-t.y
    let p = VariableType::Variable;
    let t: Constant = Vec2::new(inner_radius, outer_radius).into();
    let q = vec2!(sub!(length!(xz!(p.clone())), x!(t.clone())), y!(p.clone()));

    sub!(length!(q), y!(t))
}