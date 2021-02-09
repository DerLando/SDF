use ops::{AddOp, LengthOp, NegOp, NoOp};
use sdf_vecs::{Vec3, Vec1, VecType, ops::{Length, add_high}, ComponentAccess};
use std::ops::DerefMut;

mod ops;
mod sdf;

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
trait Spatial: Operator<VecType> + VariableContainer { }
