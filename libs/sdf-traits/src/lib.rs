use dyn_clone::DynClone;
use sdf_vecs::{Vec3, VecType};

#[macro_use]
mod ops;

mod sdf;
mod primitives;

pub use sdf::TraitSDF;

trait VariableContainer {
    fn replace_variable(&mut self, var: &Vec3);
}

/// The basic operator trait all operators have to implement
trait Operator<T> {
    fn operate(&self) -> T;
}

/// Operators that always return a [`VecType`].
/// This marker trait is stored in all Ops as a Trait object
trait Spatial: Operator<VecType> + VariableContainer + DynClone { }

dyn_clone::clone_trait_object!(Spatial);
