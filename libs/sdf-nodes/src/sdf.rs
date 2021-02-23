use std::{ops::Deref, rc::Rc};

use sdf_vecs::{Vec3, VecType};

use crate::{constant::Constant, constant::ConstantContainer, node::{ArgsIterMut, BinaryNode, BinaryNodeBuilder, Node, UnaryNode}, ops::{BinaryOperator, UnaryOperator, Operator}, variable::VariableType};

pub struct SdfTree {
    constants: ConstantContainer,
    root: Node
}

impl Default for SdfTree {
    fn default() -> Self {
        Self {
            constants: ConstantContainer::default(),
            root: Node::default()
        }
    }
}

impl SdfTree {
    fn migrate_constants(node: &mut Node, constants: &mut ConstantContainer) {
        node
            .args_iter_mut()
            .for_each(|v| {
                match v {
                    VariableType::Constant(c) => *c = constants.get_or_insert(c.deref()),
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
        let length_node = UnaryNode::new(
            VariableType::Variable,
            UnaryOperator::Length
        );

        let sub_node = 
            BinaryNodeBuilder::new()
                .lhs(length_node.into())
                .rhs(VariableType::Constant(tree.constants.get_or_insert(&radius.into())))
                .op(BinaryOperator::Sub)
                .build()
                ;

        tree.root = Node::Binary(Box::new(sub_node));
        tree
    }

    pub fn union(a: Self, b: Self) -> Self {
        let root = 
            BinaryNodeBuilder::new()
                .lhs(VariableType::Node(a.root))
                .rhs(VariableType::Node(b.root))
                .op(BinaryOperator::Min)
                .build()
                ;

        let constants = ConstantContainer::default();

        let mut union = Self {
            constants,
            root: Node::Binary(Box::new(root))
        };

        Self::migrate_constants(&mut union.root, &mut union.constants);

        union
    }

    pub fn scale(mut sdf: Self, factor: f32) -> Self {
        let s = sdf.constants.get_or_insert(&factor.into());

        // wrap root in mul op
        let root = 
        Node::Binary(BinaryNodeBuilder::new()
            .lhs(VariableType::Node(sdf.root))
            .rhs(VariableType::Constant(s))
            .op(BinaryOperator::Mul)
            .scale(factor)
            .build()
            .into())
            ;

        Self {
            constants: sdf.constants,
            root
        }
    }
}