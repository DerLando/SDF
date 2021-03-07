use std::{fmt::Display, ops::Deref, rc::Rc};

use sdf_vecs::{RotationAxis, TransformHelper, Vec3, VecType, ops::Length};

use crate::{constant::Constant, csg::{difference, intersection, union, union_smooth}, node::{BinaryNode, BinaryNodeBuilder, Node, TransformMut, UnaryNode}, ops::{BinaryOperator, UnaryOperator, Operator}, primitives::{box_2d, box_3d, sphere}, simplify::{SimplificationFolder}, variable::VariableType};

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

impl From<Node> for SdfTree {
    fn from(root: Node) -> Self {
        Self{root}
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
    pub fn point() -> Self {
        length!(VariableType::Variable).into()
    }

    pub fn circle(radius: f32) -> Self {
        sphere(radius).into()
    }

    pub fn sphere(radius: f32) -> Self {
        Self::circle(radius)
    }

    pub fn rectangle(width: f32, height: f32) -> Self {
        box_2d(width, height).into()
    }

    pub fn cuboid(x: f32, y: f32, z: f32) -> Self {
        box_3d(x, y, z).into()
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

    pub fn translate(&mut self, v: &Vec3) {
        // apply inverse of translation AFTER the original transformation
        let transform = self.root.transform_mut().clone();
        *self.root.transform_mut() = TransformHelper::translation(v).inverse() * transform;
    }

    pub fn rotate(&mut self, angle: f32, axis: RotationAxis) {
        let transform = self.root.transform_mut().clone();
        *self.root.transform_mut() = TransformHelper::rotate(angle, axis).inverse() * transform;
    }
}
