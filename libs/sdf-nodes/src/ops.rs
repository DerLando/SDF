use std::ops::Deref;

use sdf_vecs::{Vec3, VecType, ops::{Length, min_high, mul_high}};

use crate::{node::{Args, BinaryNode, UnaryNode}, variable::VariableType};

pub(crate) trait Operator {
    fn operate(&self, sample: &Vec3) -> VecType;
}

pub(crate) enum UnaryOperator {
    Length,
    NoOp
}

pub(crate) enum BinaryOperator {
    Sub,
    Min,
    Mul
}

pub(crate) enum TernaryOperator {

}

pub(crate) enum QuaternaryOperator {

}

impl Operator for VariableType {
    fn operate(&self, sample: &Vec3) -> VecType {
        match self {
            VariableType::Variable => VecType::Vec3(*sample),
            VariableType::Constant(c) => *c.deref(),
            VariableType::Node(n) => n.operate(sample)
        }
    }
}

macro_rules! impl_unary_op {
    ($name:ident, $closure:expr) => {
        pub(crate) fn $name(node: &UnaryNode, sample: &Vec3) -> VecType {
            ($closure)(
            node.args()[0]
                .operate(sample)
            )
        }
    };
}

impl_unary_op!(length, |v: VecType| v.length().into());

macro_rules! impl_binary_op {
    ($name:ident, $closure:expr) => {
        pub(crate) fn $name(node: &BinaryNode, sample: &Vec3) -> VecType {
            let args = node.args();
            ($closure)(
                (args[0].operate(sample), args[1].operate(sample))
            )
        }
    };
}

impl_binary_op!(sub, |(a, b)| a - b);
impl_binary_op!(min, |(a, b)| min_high(&a, &b));
impl_binary_op!(mul, |(a, b)| mul_high(&a, &b));