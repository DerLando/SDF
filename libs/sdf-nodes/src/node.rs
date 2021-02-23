use std::ops::{Deref, DerefMut};

use sdf_vecs::{Vec3, VecType};

use crate::{ops::{BinaryOperator, Operator, QuaternaryOperator, TernaryOperator, UnaryOperator, length, sub, min, mul}, variable::VariableType};

pub(crate) struct UnaryNode {
    args: [VariableType; 1],
    op: UnaryOperator,
    scale: f32
}

impl UnaryNode {
    pub fn new(arg: VariableType, op: UnaryOperator) -> Self {
        Self {
            args: [arg],
            op,
            scale: 1.0
        }
    }
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

pub(crate) struct BinaryNode {
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

pub(crate) struct TernaryNode {
    args: [VariableType; 3],
    op: TernaryOperator
}

impl Operator for TernaryNode {
    fn operate(&self, sample: &Vec3) -> VecType {
        todo!()
    }
}

pub(crate) struct QuaternaryNode {
    args: [VariableType; 4],
    op: QuaternaryOperator
}

impl Operator for QuaternaryNode {
    fn operate(&self, sample: &Vec3) -> VecType {
        todo!()
    }
}

pub(crate) struct BinaryNodeBuilder {
    lhs: VariableType,
    rhs: VariableType,
    op: BinaryOperator,
    scale: f32
}

impl BinaryNodeBuilder {
    pub fn new() -> Self {
        Self {
            lhs: VariableType::Variable,
            rhs: VariableType::Variable,
            op: BinaryOperator::Min,
            scale: 1.0
        }
    }

    pub fn lhs(mut self, lhs: VariableType) -> Self {
        self.lhs = lhs;
        self
    }

    pub fn rhs(mut self, rhs: VariableType) -> Self {
        self.rhs = rhs;
        self
    }

    pub fn op(mut self, op: BinaryOperator) -> Self {
        self.op = op;
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn build(self) -> BinaryNode {
        BinaryNode {
            args: [self.lhs, self.rhs],
            op: self.op,
            scale: self.scale
        }
    }
}

pub(crate) enum Node {
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

impl Default for Node {
    fn default() -> Self {
        Node::Unary(Box::new(UnaryNode{args: [VariableType::Variable], op: UnaryOperator::NoOp, scale: 1.0}))
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

pub(crate) trait ArgsIter {
    fn args_iter(&self) -> std::slice::Iter<VariableType>;
}

pub(crate) trait ArgsIterMut {
    fn args_iter_mut(&mut self) -> std::slice::IterMut<VariableType>;
}

macro_rules! impl_args_getters {
    ($name: ident) => {
        impl Args for $name {
            fn args(&self) -> &[VariableType] {
                &self.args
            }
        }

        impl ArgsIter for $name {
            fn args_iter(&self) -> std::slice::Iter<VariableType> {
                self.args.iter()
            }
        }

        impl ArgsIterMut for $name {
            fn args_iter_mut(&mut self) -> std::slice::IterMut<VariableType> {
                self.args.iter_mut()
            }
        }
    };
}

impl_args_getters!(UnaryNode);
impl_args_getters!(BinaryNode);
impl_args_getters!(TernaryNode);
impl_args_getters!(QuaternaryNode);

impl ArgsIter for Node {
    fn args_iter(&self) -> std::slice::Iter<VariableType> {
        match self {
            Node::Unary(n) => n.deref().args_iter(),
            Node::Binary(n) => n.deref().args_iter(),
            Node::Ternary(n) => n.deref().args_iter(),
            Node::Quaternary(n) => n.deref().args_iter(),
        }
    }
}

impl ArgsIterMut for Node {
    fn args_iter_mut(&mut self) -> std::slice::IterMut<VariableType> {
        match self {
            Node::Unary(n) => n.deref_mut().args_iter_mut(),
            Node::Binary(n) => n.deref_mut().args_iter_mut(),
            Node::Ternary(n) => n.deref_mut().args_iter_mut(),
            Node::Quaternary(n) => n.deref_mut().args_iter_mut(),
        }
    }
}

pub(crate) trait Args {
    fn args(&self) -> &[VariableType];
}