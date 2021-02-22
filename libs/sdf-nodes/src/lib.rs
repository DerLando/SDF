use std::{collections::HashSet, ops::{Deref, DerefMut, Index}, rc::Rc};

use sdf_vecs::{Vec3, VecType, ops::{Length, min_high, mul_high}};

pub struct SdfTree {
    constants: Vec<VecType>,
    root: Node
}

impl Default for SdfTree {
    fn default() -> Self {
        Self {
            constants: Vec::new(),
            root: Node::Unary(Box::new(UnaryNode{args: [VariableType::Variable], op: UnaryOperator::NoOp, scale: 1.0}))
        }
    }
}

impl SdfTree {
    fn insert_const(constants: &mut Vec<VecType>, value: &VecType) -> Constant {
        if let Some(c) = constants.iter().find(|c| *c == value) {
            Rc::new(*c)
        } else {
            constants.push(*value);
            Rc::new(*constants.last().unwrap())
        }
    }

    fn get_const(&mut self, value: &VecType) -> Constant {
        Self::insert_const(&mut self.constants, value)
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
            op: UnaryOperator::Length,
            scale: 1.0
        };
        let sub_node = BinaryNode {
            args: [
                VariableType::Node(Node::Unary(Box::new(length_node))),
                VariableType::Constant(tree.get_const(&radius.into()))
            ],
            op: BinaryOperator::Sub,
            scale: 1.0
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
            op: BinaryOperator::Min,
            scale: 1.0
        };

        let constants: Vec<_> = Vec::with_capacity(a.constants.len() + b.constants.len());

        let mut union = Self {
            constants,
            root: Node::Binary(Box::new(root))
        };

        Self::migrate_constants(&mut union.root, &mut union.constants);

        union
    }

    pub fn scale(mut sdf: Self, factor: f32) -> Self {
        let s = sdf.get_const(&factor.into());

        // wrap root in mul op
        let root = Node::Binary(Box::new(BinaryNode {
            args: [
                VariableType::Node(sdf.root),
                VariableType::Constant(s)
            ],
            op: BinaryOperator::Mul,
            scale: factor
        }));

        Self {
            constants: sdf.constants,
            root
        }
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

fn mul(node: &BinaryNode, sample: &Vec3) -> VecType {
    let lhs = node.args[0].operate(sample);
    let rhs = node.args[1].operate(sample);

    mul_high(&lhs, &rhs)
}

enum UnaryOperator {
    Length,
    NoOp
}

enum BinaryOperator {
    Sub,
    Min,
    Mul
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
    op: UnaryOperator,
    scale: f32
}

impl Operator for UnaryNode {
    fn operate(&self, sample: &Vec3) -> VecType {
        let mut p: Vec3 = *sample;

        // test if we need to compress space
        if self.scale != 1.0 {p = p / self.scale};
        
        match self.op {
            UnaryOperator::Length => length(self, &p),
            UnaryOperator::NoOp => VecType::Vec3(p),
        }
    }
}

struct BinaryNode {
    args: [VariableType; 2],
    op: BinaryOperator,
    scale: f32
}

impl Operator for BinaryNode {
    fn operate(&self, sample: &Vec3) -> VecType {
        let mut p: Vec3 = *sample;

        // test if we need to compress space
        if self.scale != 1.0 {p = p / self.scale};


        match self.op {
            BinaryOperator::Sub => sub(self, &p),
            BinaryOperator::Min => min(self, &p),
            BinaryOperator::Mul => mul(self, &p),
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

impl Node {
    fn need_compress_space(&self) -> bool {
        match self {
            Node::Unary(n) => n.scale != 1.0,
            Node::Binary(n) => n.scale != 1.0,
            _ => false
        }
    }
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

        let scale = SdfTree::circle(1.0);
        let scale = SdfTree::scale(scale, 2.0);
        assert_eq!(0.0, scale.sign_at(&Vec3::new(2.0, 0.0, 0.0)));
        let scale = SdfTree::scale(scale, 0.5);
        assert_eq!(0.0, scale.sign_at(&Vec3::new(1.0, 0.0, 0.0)));

    }
}
