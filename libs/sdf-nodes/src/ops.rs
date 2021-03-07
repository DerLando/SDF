use std::{fmt::Display, ops::Deref, rc::Rc};

use sdf_vecs::{Vec3, VecType, ComponentAccess, ops::{Length, Abs, MaxComp, min_high, mul_high, max_high, div_high, add_high, dot_high, Clamp}};

use crate::{node::{Args, BinaryNode, UnaryNode, TernaryNode, QuaternaryNode, Node}, variable::VariableType};

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
    Dot,
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
            BinaryOperator::Dot => write!(f, "dot"),
        }
    }
}

pub(crate) enum TernaryOperator {
    Clamp
}

impl Display for TernaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TernaryOperator::Clamp => write!(f, "clamp"),
        }
    }
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
impl_binary_op!(dot, |(a, b)| dot_high(&a, &b));

macro_rules! impl_ternary_op {
    ($name:ident, $closure:expr) => {
        paste::item! {
            pub(crate) fn [<$name _op>](node: &TernaryNode, sample: &Vec3) -> VecType {
                let args = node.args();
                ($closure)(
                    (args[0].operate(sample), args[1].operate(sample), args[2].operate(sample))
                )
            }

            macro_rules! $name {
                ($a:expr, $b:expr, $c:expr) => {
                    {
                        let node = 
                        TernaryNodeBuilder::new()
                            .a($a.into())
                            .b($b.into())
                            .c($c.into())
                            .op(TernaryOperator::[<$name:camel>])
                            .build()
                            ;
                        Node::Ternary(Rc::new(node))
                    }
                }
            }            
        }
    };
}

impl_ternary_op!(clamp, |(a, b, c)| VecType::clamp(&a, VecType::x(&b), VecType::x(&c)));

macro_rules! impl_quaternary_op {
    ($name:ident, $closure:expr) => {
        paste::item! {
            pub(crate) fn [<$name _op>](node: &QuaternaryNode, sample: &Vec3) -> VecType {
                let args = node.args();
                ($closure)(
                    (args[0].operate(sample), args[1].operate(sample), args[2].operate(sample), args[3].operate(sample))
                )
            }

            macro_rules! $name {
                ($a:expr, $b:expr, $c:expr, $d: expr) => {
                    {
                        let node = 
                        QuaternaryNodeBuilder::new()
                            .a($a.into())
                            .b($b.into())
                            .c($c.into())
                            .d($d.into())
                            .op(QuaternaryOperator::[<$name:camel>])
                            .build()
                            ;
                        Node::Quaternary(Rc::new(node))
                    }
                }
            }            
        }
    };
}
