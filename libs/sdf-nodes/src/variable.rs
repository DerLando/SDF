use std::fmt::Display;

use sdf_vecs::VecType;

use crate::{constant::Constant, node::{BinaryNode, Node, UnaryNode}};

pub(crate) struct Variable;

pub(crate) enum VariableType {
    Variable,
    Constant(Constant),
    Node(Node)
}

impl Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::Variable => write!(f, "p"),
            VariableType::Constant(c) => write!(f, "{}", *c),
            VariableType::Node(n) => write!(f, "{}", *n)
        }
    }
}

impl VariableType {
    pub(crate) fn is_variable(&self) -> bool {
        match self {
            VariableType::Variable => true,
            _ => false
        }
    }

    pub(crate) fn is_const(&self) -> bool {
        match self {
            VariableType::Constant(_) => true,
            _ => false
        }
    }

    pub(crate) fn is_node(&self) -> bool {
        match self {
            VariableType::Node(_) => true,
            _ => false
        }
    }
}

impl From<UnaryNode> for VariableType {
    fn from(arg: UnaryNode) -> Self {
        VariableType::Node(Node::Unary(Box::new(arg)))
    }
}

impl From<BinaryNode> for VariableType {
    fn from(arg: BinaryNode) -> Self {
        VariableType::Node(Node::Binary(Box::new(arg)))
    }
}

impl From<VecType> for VariableType {
    fn from(arg: VecType) -> Self {
        VariableType::Constant(arg)
    }
}

impl From<f32> for VariableType {
    fn from(arg: f32) -> Self {
        VariableType::Constant(arg.into())
    }
}