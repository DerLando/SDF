use sdf_vecs::{Vec3, Vec1, VecType, ops::{Length, add_high}, ComponentAccess};
use std::ops::DerefMut;

enum Variable {
    Const(VecType),
    Replaceable(Vec3)
}

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

struct NoOp(Variable);

impl Spatial for NoOp {}

impl VariableContainer for NoOp {
    fn replace_variable(&mut self, var: &Vec3) {
        match &mut self.0 {
            Variable::Replaceable(r) => *r = *var,
            Variable::Const(_) => ()
        }
    }
}

impl Operator<VecType> for NoOp {
    fn operate(&self) -> VecType {
        match self.0 {
            Variable::Replaceable(r) => VecType::Vec3(r),
            Variable::Const(c) => c
        }
    }
}

impl NoOp {
    fn new_const(c: &VecType) -> Self {
        Self(Variable::Const(*c))
    }

    fn new_var() -> Self {
        Self(Variable::Replaceable(Vec3::default()))
    }
}

struct AddOp<L, R> where L: Spatial, R: Spatial{
    lhs: L,
    rhs: R
}

impl<L, R> Spatial for AddOp<L, R> where L: Spatial, R: Spatial {}

impl<L, R> VariableContainer for AddOp<L, R> where L: Spatial, R: Spatial {
    fn replace_variable(&mut self, var: &Vec3) {
        self.lhs.replace_variable(var);
        self.rhs.replace_variable(var);
    }
}

impl<L, R> Operator<VecType> for AddOp<L, R> where L: Spatial, R: Spatial {
    fn operate(&self) -> VecType {
        add_high(&self.lhs.operate(), &self.rhs.operate())
    }
}

struct LengthOp<S: Spatial>(S);

impl<S> Spatial for LengthOp<S> where S: Spatial { }

impl<S> VariableContainer for LengthOp<S>
where S: Spatial {
    fn replace_variable(&mut self, var: &Vec3) {
        self.0.replace_variable(var);
    }
}

impl<S> Operator<VecType> for LengthOp<S>
where S: Spatial {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().length()).into()
    }
}

struct NegOp<S: Spatial>(S);

impl<S> Spatial for NegOp<S> where S: Spatial { }

impl<S> VariableContainer for NegOp<S> where S: Spatial {
    fn replace_variable(&mut self, var: &Vec3) {
        self.0.replace_variable(var)
    }
}

impl<S> Operator<VecType> for NegOp<S> where S: Spatial {
    fn operate(&self) -> VecType {
        -self.0.operate()
    }
}

pub struct TraitSDF {
    root: Box<dyn Spatial>
}

impl TraitSDF {
    pub fn sign_at(&mut self, position: &Vec3) -> f32 {
        // insert position for variable
        self.root.deref_mut().replace_variable(position);

        // operate the whole tree and return
        match self.root.operate() {
            VecType::Vec1(v) => v.x(),
            _ => unreachable!()
        }

    }

    fn var_length() -> Self {
        let length = LengthOp(NoOp::new_var());

        Self {
            root: Box::new(length)
        }
    }

    pub fn circle(center: &Vec3, radius: f32) -> Self {
        // length(P-C)-r, where P is query point, C is Center vec and r is radius
        let center_neg = NegOp(NoOp::new_const(&VecType::Vec3(*center)));
        let center_var_sub = AddOp {
            lhs: NoOp::new_var(),
            rhs: center_neg,
        };
        let dist_from_center = LengthOp(center_var_sub);
        let radius_neg = NegOp(NoOp::new_const(&VecType::Vec1(Vec1::new(radius))));
        let radius_sub = AddOp {
            lhs: dist_from_center,
            rhs: radius_neg
        };

        Self {
            root: Box::new(radius_sub)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_works() {
        let mut sdf = TraitSDF::var_length();

        assert_eq!(1.0, sdf.sign_at(&Vec3::new(1.0, 0.0, 0.0)));
        assert_eq!(1.0, sdf.sign_at(&Vec3::new(-1.0, 0.0, 0.0)));
        assert_eq!(2.0f32.sqrt(), sdf.sign_at(&Vec3::new(-1.0, 1.0, 0.0)));
    }

    #[test]
    fn circle_works() {
        let mut sdf = TraitSDF::circle(&Vec3::new(0.0, -1.0, 0.0), 10.0);

        assert_eq!(-10.0, sdf.sign_at(&Vec3::new(0.0, -1.0, 0.0)));
        assert_eq!(0.0, sdf.sign_at(&Vec3::new(10.0, -1.0, 0.0)));
        assert_eq!(10.0, sdf.sign_at(&Vec3::new(20.0, -1.0, 0.0)));
    }
}
