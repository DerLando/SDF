use crate::{constant::Constant, node::{BinaryNode, Node, UnaryNode}};

pub(crate) struct Variable;

pub(crate) enum VariableType {
    Variable,
    Constant(Constant),
    Node(Node)
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