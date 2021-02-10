use std::ops::Deref;

use crate::{Spatial, VariableContainer};
use sdf_vecs::Vec3;

/// Implement the boilerplate for an unary operator (single argument).
/// You will still need to impl Operator<VecType> to satify the Spatial impl.
macro_rules! impl_unary_op {
    ($op_name:ident, $fn_name:ident) => {
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

        pub(crate)fn $fn_name(sdf: impl Spatial + 'static) -> $op_name {
            $op_name::new(Box::new(sdf))
        }

    };  

}

macro_rules! create_dsl_unary {
    ($op_name:ident) => {
        macro_rules! $op_name {
            ($sdf:expr) => {
                {
                    $op_name::new(Box::new($sdf))
                }
            }
        }
    };
}


macro_rules! impl_binary_op {
    ($op_name:ident, $fn_name:ident) => {
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

        pub(crate)fn $fn_name(lhs: impl Spatial + 'static, rhs: impl Spatial + 'static) -> $op_name {
            $op_name::new(Box::new(lhs), Box::new(rhs))
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
mod vecop;

pub(crate) use self::noop::NoOp;
pub(crate) use self::add::add;
pub(crate) use self::neg::neg;
pub(crate) use self::length::length;
pub(crate) use self::sub::sub;
pub(crate) use self::swizzle::{X, Y, Z, W, XX, XY, YY, YX, x, y, xx, xy, yy, yx};
pub(crate) use self::abs::{Abs, abs};
pub(crate) use self::max::max;
pub(crate) use self::min::min;
pub(crate) use self::max_comp::{MaxComp, max_comp};
pub(crate) use self::mul::mul;
pub(crate) use self::vecop::vec2;
