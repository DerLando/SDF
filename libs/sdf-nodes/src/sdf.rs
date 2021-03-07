use std::{fmt::Display, ops::Deref, rc::Rc};

use sdf_vecs::{Vec3, VecType};

use crate::{constant::Constant, node::{BinaryNode, BinaryNodeBuilder, Node, UnaryNode}, ops::{BinaryOperator, UnaryOperator, Operator}, simplify::{SimplificationFolder}, variable::VariableType};

pub struct SdfTree {
    root: Node
}

impl Display for SdfTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl Default for SdfTree {
    fn default() -> Self {
        Self {
            root: Node::default()
        }
    }
}

impl SdfTree {
    pub fn sign_at(&self, sample: &Vec3) -> f32 {
        match self.root.operate(sample) {
            VecType::Scalar(s) => s,
            _ => unreachable!()
        }
    }

    pub fn simplify(&mut self) {
        let mut simplifier = SimplificationFolder;
        self.root = simplifier.simplify(&self.root);
    }

    pub fn circle(radius: f32) -> Self {
        let r: VariableType = radius.into();
        let root = sub!(length!(VariableType::Variable), r);

        match root {
            VariableType::Node(n) => Self {root: n},
            _ => unreachable!()
        }
    }

    pub fn union(a: Self, b: Self) -> Self {
        let root = 
            BinaryNodeBuilder::new()
                .lhs(VariableType::Node(a.root))
                .rhs(VariableType::Node(b.root))
                .op(BinaryOperator::Min)
                .build()
                ;

        let mut union = Self {
            root: Node::Binary(Rc::new(root))
        };

        union
    }

    pub fn scale(&mut self, factor: f32) {
        let s: Constant = factor.into();

        // wrap root in mul op
        self.root = 
        Node::Binary(BinaryNodeBuilder::new()
            .lhs(VariableType::Node(self.root.clone()))
            .rhs(VariableType::Constant(s))
            .op(BinaryOperator::Mul)
            .scale(factor)
            .build()
            .into())
            ;
    }
}