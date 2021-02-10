#[macro_use]

use dyn_clone::DynClone;
use sdf_vecs::{Vec3, VecType};

mod ops;
mod sdf;
mod primitives;

pub use sdf::TraitSDF;

pub trait VariableContainer {
    fn replace_variable(&mut self, var: &Vec3);
}

/// The basic operator trait all operators have to implement
pub trait Operator<T> {
    fn operate(&self) -> T;
}

/// Operators that always return a [`VecType`].
/// This marker trait is stored in all Ops as a Trait object
pub trait Spatial: Operator<VecType> + VariableContainer + DynClone { }

dyn_clone::clone_trait_object!(Spatial);
