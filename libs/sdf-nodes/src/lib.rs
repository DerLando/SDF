use std::{collections::HashSet, ops::{Deref, DerefMut, Index}, rc::Rc};

use sdf_vecs::{Vec3, VecType, ops::{Length, min_high}};

pub struct SdfTree {
    constants: Vec<VecType>,
    root: Node
}

impl Default for SdfTree {
    fn default() -> Self {
        Self {
            constants: Vec::new(),
            root: Node::Unary(Box::new(UnaryNode{args: [VariableType::Variable], op: UnaryOperator::NoOp}))
        }
    }
}

impl SdfTree {
    fn insert_const(constants: &mut Vec<VecType>, value: &VecType) -> Constant {
        constants.push(*value);
        Rc::new(*constants.last().unwrap())
    }

    fn get_const(&mut self, value: &VecType) -> Constant {
        if let Some(c) = self.constants.iter().find(|c| *c == value) {
            Rc::new(*c)
        } else {
            Self::insert_const(&mut self.constants, value)
        }
    }

    fn migrate_constants(node: &mut Node, constants: &mut Vec<VecType>) {
        node
            .iter_mut()
            .for_each(|v| {
                match v {
                    VariableType::Constant(c) => *c = Self::insert_const(constants, c.deref()),
                    _ => ()
                }
            });

    }

    pub fn sign_at(&self, sample: &Vec3) -> f32 {
        match self.root.operate(sample) {
            VecType::Scalar(s) => s,
            _ => unreachable!()
        }
    }

    pub fn circle(radius: f32) -> Self {
        // set up tree
        let mut tree = Self::default();

        // set up nodes
        let length_node = UnaryNode {
            args: [VariableType::Variable],
            op: UnaryOperator::Length
        };
        let sub_node = BinaryNode {
            args: [
                VariableType::Node(Node::Unary(Box::new(length_node))),
                VariableType::Constant(tree.get_const(&radius.into()))
            ],
            op: BinaryOperator::Sub
        };

        tree.root = Node::Binary(Box::new(sub_node));
        tree
    }

    pub fn union(a: Self, b: Self) -> Self {
        let root = BinaryNode {
            args: [
                VariableType::Node(a.root),
                VariableType::Node(b.root)
            ],
            op: BinaryOperator::Min
        };

        let constants: Vec<_> = Vec::with_capacity(a.constants.len() + b.constants.len());

        let mut union = Self {
            constants,
            root: Node::Binary(Box::new(root))
        };

        Self::migrate_constants(&mut union.root, &mut union.constants);

        union
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

fn min(node: &BinaryNode, sample: &Vec3) -> VecType {
    let lhs = node.args[0].operate(sample);
    let rhs = node.args[1].operate(sample);

    min_high(&lhs, &rhs)
}

enum UnaryOperator {
    Length,
    NoOp
}

enum BinaryOperator {
    Sub,
    Min
}

enum TernaryOperator {

}

enum QuaternaryOperator {

}

trait Operator {
    fn operate(&self, sample: &Vec3) -> VecType;
}

struct Variable;

type Constant = Rc<VecType>;

struct UnaryNode {
    args: [VariableType; 1],
    op: UnaryOperator
}

impl Operator for UnaryNode {
    fn operate(&self, sample: &Vec3) -> VecType {
        match self.op {
            UnaryOperator::Length => length(self, sample),
            UnaryOperator::NoOp => VecType::Vec3(*sample),
        }
    }
}

struct BinaryNode {
    args: [VariableType; 2],
    op: BinaryOperator
}

impl Operator for BinaryNode {
    fn operate(&self, sample: &Vec3) -> VecType {
        match self.op {
            BinaryOperator::Sub => sub(self, sample),
            BinaryOperator::Min => min(self, sample)
        }
    }
}

struct TernaryNode {
    args: [VariableType; 3],
    op: TernaryOperator
}

impl Operator for TernaryNode {
    fn operate(&self, sample: &Vec3) -> VecType {
        todo!()
    }
}

struct QuaternaryNode {
    args: [VariableType; 4],
    op: QuaternaryOperator
}

impl Operator for QuaternaryNode {
    fn operate(&self, sample: &Vec3) -> VecType {
        todo!()
    }
}

enum Node {
    Unary(Box<UnaryNode>),
    Binary(Box<BinaryNode>),
    Ternary(Box<TernaryNode>),
    Quaternary(Box<QuaternaryNode>)
}

impl Operator for Node {
    fn operate(&self, sample: &Vec3) -> VecType {
        match self {
            Node::Unary(n) => n.deref().operate(sample),
            Node::Binary(n) => n.deref().operate(sample),
            Node::Ternary(n) => n.deref().operate(sample),
            Node::Quaternary(n) => n.deref().operate(sample),
        }
    }
}

trait VariableContainer {
    fn iter_mut(&mut self) -> std::slice::IterMut<VariableType>;
}

impl VariableContainer for Node {
    fn iter_mut(&mut self) -> std::slice::IterMut<VariableType> {
        match self {
            Node::Unary(n) => n.deref_mut().args.iter_mut(),
            Node::Binary(n) => n.deref_mut().args.iter_mut(),
            Node::Ternary(n) => n.deref_mut().args.iter_mut(),
            Node::Quaternary(n) => n.deref_mut().args.iter_mut(),
        }
    }
}

enum VariableType {
    Variable,
    Constant(Constant),
    Node(Node)
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let tree = SdfTree::circle(1.0);

        assert_eq!(-1.0, tree.sign_at(&Vec3::default()));
        assert_eq!(0.0, tree.sign_at(&Vec3::new(1.0, 0.0, 0.0)));
        assert_eq!(1.0, tree.sign_at(&Vec3::new(2.0, 0.0, 0.0)));

        let other = SdfTree::circle(2.0);

        let union = SdfTree::union(tree, other);

        assert_eq!(0.0, union.sign_at(&Vec3::new(2.0, 0.0, 0.0)));
    }
}
