use std::{fmt::Display, ops::Deref, rc::Rc};

use sdf_vecs::{Vec3, VecType};

use crate::{constant::Constant, csg::{difference, intersection, union, union_smooth}, node::{BinaryNode, BinaryNodeBuilder, Node, UnaryNode}, ops::{BinaryOperator, UnaryOperator, Operator}, simplify::{SimplificationFolder}, variable::VariableType};

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

/// general purpose
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
}

/// primitives
impl SdfTree {
    pub fn circle(radius: f32) -> Self {
        let r: Constant = radius.into();
        let root = sub!(length!(VariableType::Variable), r);

        Self {
            root
        }
    }
}

/// CSG
impl SdfTree {
    pub fn union(a: Self, b: Self) -> Self {
        Self{root: union(a.root, b.root)}
    }

    pub fn intersection(a: Self, b: Self) -> Self {
        Self{root: intersection(a.root, b.root)}
    }

    pub fn difference(a: Self, b: Self) -> Self {
        Self{root: difference(a.root, b.root)}
    }

    pub fn union_smooth(a: Self, b: Self, smooth_fac: f32) -> Self {
        Self{root: union_smooth(a.root, b.root, smooth_fac)}
    }
}

/// Transforms
impl SdfTree {
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
