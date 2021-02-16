#[macro_use]

use dyn_clone::DynClone;
use glam::Vec3;
use sdf_vecs::{VecType};

mod ops;
mod sdf;
mod primitives;
mod csg;
mod transform;

pub use sdf::SDFTree;

pub trait VariableContainer {
    fn replace_variable(&mut self, var: &Vec3);
}

/// The basic operator trait all operators have to implement
pub trait Operator {
    fn operate(&self, pos: &Vec3) -> VecType;
}

/// Operators that always return a [`VecType`].
/// This marker trait is stored in all Ops as a Trait object
pub trait Spatial: Operator + DynClone { }

dyn_clone::clone_trait_object!(Spatial);
