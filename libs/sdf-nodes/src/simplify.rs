use std::{rc::Rc};

use sdf_vecs::Vec3;

use crate::{constant::Constant, node::{ArgsIter, BinaryNode, FoldArgs, Node, UnaryNode}, ops::{Operator, UnaryOperator}, variable::VariableType};

trait Fold {
    fn fold_constant(&mut self, c: Constant) -> Constant {c.into()}
    fn fold_variable(&mut self) -> VariableType {VariableType::Variable}
    /// override this to manipulate nodes,
    /// call self.fold_node() on the manipulated node at the end
    fn fold_node_type(&mut self, node: &Node) -> Node {self.fold_node(node)}
    fn fold_node(&mut self, node: &Node) -> Node {
        match node {
            Node::Unary(n) => Node::Unary(Rc::new(self.fold_unary(n.clone()))),
            Node::Binary(n) => Node::Binary(Rc::new(self.fold_binary(n.clone()))),
            _ => unreachable!()
        }
    }
    fn fold_unary(&mut self, node: Rc<UnaryNode>) -> UnaryNode {
        let args = node.args_iter().map(|v| self.fold_variable_type(v));
        node.fold_args(args)
    }
    fn fold_binary(&mut self, node: Rc<BinaryNode>) -> BinaryNode {
        let args = node.args_iter().map(|v| self.fold_variable_type(v));
        node.fold_args(args)
    }
    fn fold_variable_type(&mut self, v: &VariableType) -> VariableType {
        match v {
            VariableType::Constant(c) => self.fold_constant(*c).into(),
            VariableType::Variable => self.fold_variable(),
            VariableType::Node(n) => VariableType::Node(self.fold_node_type(n))
        }
    }
}

pub(crate) struct SimplificationFolder;

impl Fold for SimplificationFolder {
    fn fold_node_type(&mut self, node: &Node) -> Node {
        match simplify(node) {
            Some(n) => self.fold_node(&n),
            None => self.fold_node(node)
        }
    }
}

impl SimplificationFolder {
    pub(crate) fn simplify(&mut self, node: &Node) -> Node {
        self.fold_node_type(node)
    }
}

/// recursively traverse the node tree, to test if it contains a variable.
/// If so, it can't ne simplified, as variables have to be kept.
fn can_simplify(node: &Node) -> bool {
    fn can_simplify_variabletype(v: &VariableType) -> bool {
        match v {
            VariableType::Constant(_) => true,
            VariableType::Variable => false,
            VariableType::Node(n) => can_simplify(n)
        }
    }

    node.args_iter().all(|v| can_simplify_variabletype(v))
}

/// Simplificaiton if super simple, just evaluate the node at any point (as it's constant the point doesn't matter),
/// And store the result as a UnaryNode, with NoOp and result as args.
fn simplify(node: &Node) -> Option<Node> {
    if !can_simplify(&node) {None}
    else {
        let simp_node = UnaryNode::new(node.operate(&Vec3::default()).into(), UnaryOperator::NoOp);
        Some(Node::Unary(Rc::new(simp_node)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{node::BinaryNodeBuilder, ops::BinaryOperator};

    use super::*;

    #[test]
    fn const_ops_simplify() {
        let sub_node =
        BinaryNodeBuilder::new()
            .lhs(VariableType::Variable)
            .rhs(2.0.into())
            .op(BinaryOperator::Sub)
            .build()
            ;

        let node = Node::Binary(Rc::new(sub_node));

        let const_node = Node::Binary(Rc::new(
        BinaryNodeBuilder::new()
            .lhs(1.5.into())
            .rhs(2.0.into())
            .op(BinaryOperator::Sub)
            .build()
        ));

        let const_node = Node::Binary(Rc::new(
        BinaryNodeBuilder::new()
            .lhs(VariableType::Node(const_node))
            .rhs(3.0.into())
            .op(BinaryOperator::Mul)
            .build()
        ));

        let node = Node::Binary(Rc::new(
            BinaryNodeBuilder::new()
                .lhs(VariableType::Node(const_node))
                .rhs(VariableType::Node(node))
                .op(BinaryOperator::Min)
                .build()
        ));

        let mut simplifier = SimplificationFolder;
        let simplified = simplifier.fold_node_type(&node);

        assert_eq!(format!("{}", simplified), "min((-1.5), sub(p, 2))");
    }
}