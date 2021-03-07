use std::rc::Rc;
use sdf_vecs::{VecType, Vec3, Vec2};
use crate::{constant::Constant, node::{UnaryNode, BinaryNode, TernaryNode, TernaryNodeBuilder, BinaryNodeBuilder, Node}, variable::VariableType, ops::{UnaryOperator, BinaryOperator, TernaryOperator}};

pub(crate) fn capsule(a: Vec3, b: Vec3, radius: f32) -> Node {
    // vec3 pa = p - a, ba = b - a;
    // float h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
    // return length( pa - ba*h ) - r;

    let p: VariableType = VariableType::Variable;
    let r: Constant = radius.into();
    let zero: Constant = 0.0.into();
    let one: Constant = 1.0.into();
    let a: VecType = a.into();
    let b: VecType = b.into();

    let pa = sub!(p, a);
    let ba = sub!(b, a);
    let h = clamp!( div!(dot!(pa.clone(), ba.clone()), dot!(ba.clone(), ba.clone())), zero, one);
    
    sub!(length!(sub!(pa, mul!(ba, h))), r)
}
