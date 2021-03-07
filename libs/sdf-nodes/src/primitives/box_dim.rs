use std::rc::Rc;
use sdf_vecs::{VecType, Vec3, Vec2};
use crate::{constant::Constant, node::{UnaryNode, BinaryNode, BinaryNodeBuilder, Node}, variable::VariableType, ops::{UnaryOperator, BinaryOperator}};

fn box_nd(dimensions: VecType) -> Node {
    // R is VecN(x, y, z, w)
    // P is point to sample at
    // d = length(max(abs(P) - R, 0)) -> single quadrant not interior
    //
    // q = abs(P) - R
    // d = length(max(q,0)) + min(max(q.x, q.y), 0)

    let dim: Constant = dimensions.into();
    let zero: Constant = 0.0.into();

    let q = sub!(abs!(VariableType::Variable), dim);
    let lhs = length!(max!(q.clone(), zero.clone()));
    let rhs = min!(max_comp!(q), zero);

    add!(lhs, rhs)
}

pub(crate) fn box_2d(width: f32, height: f32) -> Node {
    box_nd(VecType::Vec2(Vec2::new(width, height)))
}

pub(crate) fn box_3d(x: f32, y: f32, z: f32) -> Node {
    box_nd(VecType::Vec3(Vec3::new(x, y, z)))
}