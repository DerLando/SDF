use std::{fmt::Display, ops::Deref, rc::Rc};

use sdf_vecs::{Vec3, VecType, ops::{Length, Abs, MaxComp, min_high, mul_high, max_high, div_high, add_high}};

use crate::{node::{Args, BinaryNode, UnaryNode, Node}, variable::VariableType};

pub(crate) trait Operator {
    fn operate(&self, sample: &Vec3) -> VecType;
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum UnaryOperator {
    Length,
    NoOp,
    Neg,
    Abs,
    MaxComp,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Length => write!(f, "length"),
            UnaryOperator::NoOp => write!(f, ""),
            UnaryOperator::Neg => write!(f, "-"),
            UnaryOperator::Abs => write!(f, "abs"),
            UnaryOperator::MaxComp => write!(f, "max_comp"),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum BinaryOperator {
    Sub,
    Min,
    Mul,
    Max,
    Div,
    Add,
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Sub => write!(f, "sub"),
            BinaryOperator::Min => write!(f, "min"),
            BinaryOperator::Mul => write!(f, "mul"),
            BinaryOperator::Max => write!(f, "max"),
            BinaryOperator::Div => write!(f, "div"),
            BinaryOperator::Add => write!(f, "add"),
        }
    }
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
        paste::item! {
            pub(crate) fn [<$name _op>](node: &UnaryNode, sample: &Vec3) -> VecType {
                ($closure)(
                node.args()[0]
                    .operate(sample)
                )
            }
            macro_rules! $name {
                ($arg:expr) => {
                    Node::Unary(std::rc::Rc::new(UnaryNode::new($arg.into(), UnaryOperator::[<$name:camel>])))
                }
            }
        }        
    };
}

impl_unary_op!(length, |v: VecType| v.length().into());
impl_unary_op!(neg, |v: VecType| (-v).into());
impl_unary_op!(abs, |v: VecType| v.abs().into());
impl_unary_op!(max_comp, |v: VecType| v.max_comp().into());

macro_rules! impl_binary_op {
    ($name:ident, $closure:expr) => {
        paste::item! {
            pub(crate) fn [<$name _op>](node: &BinaryNode, sample: &Vec3) -> VecType {
                let args = node.args();
                ($closure)(
                    (args[0].operate(sample), args[1].operate(sample))
                )
            }

            macro_rules! $name {
                ($lhs:expr, $rhs:expr) => {
                    {
                        let node = 
                        BinaryNodeBuilder::new()
                            .lhs($lhs.into())
                            .rhs($rhs.into())
                            .op(BinaryOperator::[<$name:camel>])
                            .build()
                            ;
                        Node::Binary(Rc::new(node))
                    }
                }
            }            
        }
    };
}

impl_binary_op!(sub, |(a, b)| a - b);
impl_binary_op!(min, |(a, b)| min_high(&a, &b));
impl_binary_op!(mul, |(a, b)| mul_high(&a, &b));
impl_binary_op!(max, |(a, b)| max_high(&a, &b));
impl_binary_op!(div, |(a, b)| div_high(&a, &b));
impl_binary_op!(add, |(a, b)| add_high(&a, &b));
