use std::{collections::HashSet, ops::{Deref, Index}};

use sdf_vecs::{Vec3, VecType, ops::Length};

struct SdfTree<'a> {
    constants: Vec<VecType>,
    root: Node<'a>
}

impl<'a> Default for SdfTree<'a> {
    fn default() -> Self {
        Self {
            constants: Vec::new(),
            root: Node::Unary(Box::new(UnaryNode{args: [VariableType::Variable], op: UnaryOperator::NoOp}))
        }
    }
}

impl<'a> SdfTree<'a> {
    fn get_const(&'a self, value: &VecType) -> Constant {
        self.constants.
        Constant(&self.constants[index])
    }

    pub fn circle(radius: f32) -> Self {
        // set up tree
        let mut tree = Self::default();
        tree.constants.push(radius.into());

        // set up nodes
        let length_node = UnaryNode {
            args: [VariableType::Variable],
            op: UnaryOperator::Length
        };
        let sub_node = BinaryNode {
            args: [
                VariableType::Node(Node::Unary(Box::new(length_node))),
                VariableType::Constant(tree.get_const(0))
            ],
            op: BinaryOperator::Sub
        };

        std::mem::replace(&mut tree.root, Node::Binary(Box::new(sub_node)));
        tree
    }
}

fn length(node: &UnaryNode, sample: &Vec3) -> VecType {
    node.args[0]
        .operate(sample)
        .length()
        .into()
}

fn sub(node: &BinaryNode, sample: &Vec3) -> VecType {
    let lhs = node.args[0].operate(sample);
    let rhs = node.args[1].operate(sample);

    lhs - rhs
}

enum UnaryOperator {
    Length,
    NoOp
}

enum BinaryOperator {
    Sub
}

enum TernaryOperator {

}

enum QuaternaryOperator {

}

trait Operator {
    fn operate(&self, sample: &Vec3) -> VecType;
}

struct Variable;

struct Constant<'a>(&'a VecType);

struct UnaryNode<'a> {
    args: [VariableType<'a>; 1],
    op: UnaryOperator
}

impl<'a> Operator for UnaryNode<'a> {
    fn operate(&self, sample: &Vec3) -> VecType {
        match self.op {
            UnaryOperator::Length => length(self, sample),
            UnaryOperator::NoOp => VecType::Vec3(*sample),
        }
    }
}

struct BinaryNode<'a> {
    args: [VariableType<'a>; 2],
    op: BinaryOperator
}

impl<'a> Operator for BinaryNode<'a> {
    fn operate(&self, sample: &Vec3) -> VecType {
        match self.op {
            BinaryOperator::Sub => sub(self, sample),
        }
    }
}

struct TernaryNode<'a> {
    args: [VariableType<'a>; 3],
    op: TernaryOperator
}

impl<'a> Operator for TernaryNode<'a> {
    fn operate(&self, sample: &Vec3) -> VecType {
        todo!()
    }
}

struct QuaternaryNode<'a> {
    args: [VariableType<'a>; 4],
    op: QuaternaryOperator
}

impl<'a> Operator for QuaternaryNode<'a> {
    fn operate(&self, sample: &Vec3) -> VecType {
        todo!()
    }
}

enum Node<'a> {
    Unary(Box<UnaryNode<'a>>),
    Binary(Box<BinaryNode<'a>>),
    Ternary(Box<TernaryNode<'a>>),
    Quaternary(Box<QuaternaryNode<'a>>)
}

impl<'a> Operator for Node<'a> {
    fn operate(&self, sample: &Vec3) -> VecType {
        match self {
            Node::Unary(n) => n.deref().operate(sample),
            Node::Binary(n) => n.deref().operate(sample),
            Node::Ternary(n) => n.deref().operate(sample),
            Node::Quaternary(n) => n.deref().operate(sample),
        }
    }
}

enum VariableType<'a> {
    Variable,
    Constant(Constant<'a>),
    Node(Node<'a>)
}

impl<'a> Operator for VariableType<'a> {
    fn operate(&self, sample: &Vec3) -> VecType {
        match self {
            VariableType::Variable => VecType::Vec3(*sample),
            VariableType::Constant(c) => *c.0,
            VariableType::Node(n) => n.operate(sample)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
