use std::ops::Deref;

#[macro_use]

use crate::{Spatial, VariableContainer};
use sdf_vecs::Vec3;

/// Implement the boilerplate for an unary operator (single argument).
/// You will still need to impl Operator<VecType> to satify the Spatial impl.
macro_rules! impl_unary_op {
    ($op_name:ident) => {
        #[derive(Clone)]
        pub(crate) struct $op_name(Box<dyn Spatial>);

        impl $op_name {
            pub(crate) fn new(arg: Box<dyn Spatial>) -> Self {
                Self(arg)
            }
        }

        impl Spatial for $op_name { }

        impl VariableContainer for $op_name {
            fn replace_variable(&mut self, var: &Vec3) {
                self.0.replace_variable(var)
            }
        }
    };  

}

macro_rules! impl_binary_op {
    ($op_name:ident) => {
        #[derive(Clone)]
        pub(crate) struct $op_name {
            lhs: Box<dyn Spatial>,
            rhs: Box<dyn Spatial>
        }

        impl $op_name {
            pub(crate) fn new(lhs: Box<dyn Spatial>, rhs: Box<dyn Spatial>) -> Self {
                Self {
                    lhs,
                    rhs
                }
            }
        }

        impl Spatial for $op_name { }

        impl VariableContainer for $op_name {
            fn replace_variable(&mut self, var: &Vec3) {
                self.lhs.replace_variable(var);
                self.rhs.replace_variable(var)
            }
        }
    };
}

mod noop;
mod add;
mod neg;
mod length;
mod sub;
mod swizzle;
mod abs;
mod max;
mod min;
mod max_comp;
mod mul;

pub(crate) use self::noop::NoOp;
pub(crate) use self::add::Add;
pub(crate) use self::neg::Neg;
pub(crate) use self::length::Length;
pub(crate) use self::sub::Sub;
pub(crate) use self::swizzle::{X, Y, Z, W};
pub(crate) use self::abs::Abs;
pub(crate) use self::max::Max;
pub(crate) use self::min::Min;
pub(crate) use self::max_comp::MaxComp;
pub(crate) use self::mul::Mul;
