use std::{fmt::Display, ops::{Deref, DerefMut}, rc::Rc};

use sdf_vecs::{Transform, Vec3, VecType};

use crate::{ops::{BinaryOperator, Operator, QuaternaryOperator, TernaryOperator, UnaryOperator, max_comp_op, add_op, div_op, length_op, sub_op, min_op, mul_op, max_op, neg_op, abs_op}, variable::VariableType};

pub(crate) struct UnaryNode {
    args: [VariableType; 1],
    op: UnaryOperator,
    scale: f32,
    transform: Transform
}

impl Default for UnaryNode {
    fn default() -> Self {
        UnaryNode::new(VariableType::Variable, UnaryOperator::NoOp)
    }
}

impl Display for UnaryNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.op, self.args[0])
    }
}

impl UnaryNode {
    pub fn new(arg: VariableType, op: UnaryOperator) -> Self {
        Self {
            args: [arg],
            op,
            scale: 1.0,
            transform: Transform::default()
        }
    }
}

impl Operator for UnaryNode {
    fn operate(&self, sample: &Vec3) -> VecType {
        let mut p: Vec3 = *sample;

        // test if we need to compress space
        if self.scale != 1.0 {p = p / self.scale};

        // apply transformation to position
        p = self.transform.transform_point3(p);
        
        match self.op {
            UnaryOperator::Length => length_op(self, &p),
            UnaryOperator::NoOp => VecType::Vec3(p),
            UnaryOperator::Neg => neg_op(self, &p),
            UnaryOperator::Abs => abs_op(self, &p),
            UnaryOperator::MaxComp => max_comp_op(self, &p),
        }
    }
}

pub(crate) struct BinaryNode {
    args: [VariableType; 2],
    op: BinaryOperator,
    scale: f32,
    transform: Transform
}

impl Display for BinaryNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}, {})", self.op, self.args[0], self.args[1])
    }
}

impl Operator for BinaryNode {
    fn operate(&self, sample: &Vec3) -> VecType {
        let mut p: Vec3 = *sample;

        // test if we need to compress space
        if self.scale != 1.0 {p = p / self.scale};

        // apply transformation to position
        p = self.transform.transform_point3(p);

        match self.op {
            BinaryOperator::Sub => sub_op(self, &p),
            BinaryOperator::Min => min_op(self, &p),
            BinaryOperator::Mul => mul_op(self, &p),
            BinaryOperator::Max => max_op(self, &p),
            BinaryOperator::Div => div_op(self, &p),
            BinaryOperator::Add => add_op(self, &p),
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
    scale: f32,
    transform: Transform
}

impl BinaryNodeBuilder {
    pub fn new() -> Self {
        Self {
            lhs: VariableType::Variable,
            rhs: VariableType::Variable,
            op: BinaryOperator::Min,
            scale: 1.0,
            transform: Transform::default()
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

    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn build(self) -> BinaryNode {
        BinaryNode {
            args: [self.lhs, self.rhs],
            op: self.op,
            scale: self.scale,
            transform: self.transform
        }
    }
}

#[derive(Clone)]
pub(crate) enum Node {
    Unary(Rc<UnaryNode>),
    Binary(Rc<BinaryNode>),
    Ternary(Rc<TernaryNode>),
    Quaternary(Rc<QuaternaryNode>)
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Unary(n) => write!(f, "{}", n),
            Node::Binary(n) => write!(f, "{}", n),
            _ => unreachable!()
        }
    }
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
        Node::Unary(Rc::new(UnaryNode::default()))
    }
}

impl From<VariableType> for Node {
    fn from(arg: VariableType) -> Self {
        match arg {
            VariableType::Node(n) => n,
            VariableType::Constant(c) => Node::Unary(Rc::new(UnaryNode::new(c.into(), UnaryOperator::NoOp))),
            VariableType::Variable => Node::Unary(Rc::new(UnaryNode::new(VariableType::Variable, UnaryOperator::NoOp)))
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

pub(crate) trait ArgsIter {
    fn args_iter(&self) -> std::slice::Iter<VariableType>;
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

pub(crate) trait Args {
    fn args(&self) -> &[VariableType];
}

pub(crate) trait OpAccess {
    type OpType;

    fn op(&self) -> Self::OpType;
}

impl OpAccess for UnaryNode {
    type OpType = UnaryOperator;

    fn op(&self) -> Self::OpType {
        self.op
    }
}

impl OpAccess for BinaryNode {
    type OpType = BinaryOperator;

    fn op(&self) -> Self::OpType {
        self.op
    }
}

pub(crate) trait FoldArgs {
    fn fold_args(&self, args: impl Iterator<Item = VariableType>) -> Self;
}

impl FoldArgs for UnaryNode {
    fn fold_args(&self, mut args: impl Iterator<Item = VariableType>) -> Self {
        Self::new(args.next().unwrap(), self.op)
    }
}

impl FoldArgs for BinaryNode {
    fn fold_args(&self, mut args: impl Iterator<Item = VariableType>) -> Self {
        BinaryNodeBuilder::new()
            .lhs(args.next().unwrap())
            .rhs(args.next().unwrap())
            .op(self.op)
            .build()
    }
}

/// Mutable access to transformation components
/// As the sdf tree should be immutable, it would probably be best to wrap this in an op instead
pub(crate) trait TransformMut {
    fn transform_mut(&mut self) -> &mut Transform;
}

impl TransformMut for UnaryNode {
    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl TransformMut for BinaryNode {
    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl TransformMut for Node {
    fn transform_mut(&mut self) -> &mut Transform {
        match self {
            Node::Unary(n) => Rc::get_mut(n).unwrap().transform_mut(),
            Node::Binary(n) => Rc::get_mut(n).unwrap().transform_mut(),
            _ => unreachable!()
        }
    }
}