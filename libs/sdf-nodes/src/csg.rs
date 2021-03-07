use crate::{SdfTree, constant::Constant, node::Node, node::{UnaryNode, BinaryNodeBuilder}, ops::{BinaryOperator, UnaryOperator}};
use std::rc::Rc;

pub(crate) fn union(a: Node, b: Node) -> Node {
    min!(a, b)
}

pub(crate) fn difference(a: Node, b: Node) -> Node {
    max!(a, neg!(b))
}

pub(crate) fn intersection(a: Node, b: Node) -> Node {
    max!(a, b)
}

// cubic polynomial smoothing
fn smooth_min(a: Node, b: Node, k: f32) -> Node {
    let k: Constant = k.into();
    let zero: Constant = 0.0.into();
    let one_sixth: Constant = (1.0 / 6.0).into();

    let h = div!(max!(sub!(k.clone(), abs!(sub!(a.clone(), b.clone()))), zero), k.clone());
    let h_cube = mul!(mul!(h.clone(), h.clone()), h);
    sub!(min!(a, b), mul!(mul!(h_cube, k), one_sixth))
}

pub(crate) fn union_smooth(a: Node, b: Node, k: f32) -> Node {
    smooth_min(a, b, k)
}