use std::{fmt::Display, ops::Deref, rc::Rc};

use sdf_vecs::{Vec3, VecType, ops::{Length, min_high, mul_high}};

use crate::{node::{Args, BinaryNode, UnaryNode, Node}, variable::VariableType};

pub(crate) trait Operator {
    fn operate(&self, sample: &Vec3) -> VecType;
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum UnaryOperator {
    Length,
    NoOp
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Length => write!(f, "length"),
            UnaryOperator::NoOp => write!(f, ""),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum BinaryOperator {
    Sub,
    Min,
    Mul
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Sub => write!(f, "sub"),
            BinaryOperator::Min => write!(f, "min"),
            BinaryOperator::Mul => write!(f, "mul"),
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
                    VariableType::Node(Node::Unary(Rc::new(UnaryNode::new($arg, UnaryOperator::[<$name:camel>]))))
                }
            }
        }        
    };
}

impl_unary_op!(length, |v: VecType| v.length().into());

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
                            .lhs($lhs)
                            .rhs($rhs)
                            .op(BinaryOperator::[<$name:camel>])
                            .build()
                            ;
                        VariableType::Node(Node::Binary(Rc::new(node)))
                    }
                }
            }            
        }
    };
}

impl_binary_op!(sub, |(a, b)| a - b);
impl_binary_op!(min, |(a, b)| min_high(&a, &b));
impl_binary_op!(mul, |(a, b)| mul_high(&a, &b));
