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

/// Operators that always return a VecType
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

struct AddOp {
    lhs: Box<dyn Spatial>,
    rhs: Box<dyn Spatial>
}

impl Spatial for AddOp {}

impl VariableContainer for AddOp {
    fn replace_variable(&mut self, var: &Vec3) {
        self.lhs.replace_variable(var);
        self.rhs.replace_variable(var);
    }
}

impl Operator<VecType> for AddOp {
    fn operate(&self) -> VecType {
        add_high(&self.lhs.operate(), &self.rhs.operate())
    }
}

struct LengthOp(Box<dyn Spatial>);

impl Spatial for LengthOp { }

impl VariableContainer for LengthOp {
    fn replace_variable(&mut self, var: &Vec3) {
        self.0.replace_variable(var);
    }
}

impl Operator<VecType> for LengthOp {
    fn operate(&self) -> VecType {
        Vec1::new(self.0.operate().length()).into()
    }
}

struct NegOp(Box<dyn Spatial>);

impl Spatial for NegOp { }

impl VariableContainer for NegOp {
    fn replace_variable(&mut self, var: &Vec3) {
        self.0.replace_variable(var)
    }
}

impl Operator<VecType> for NegOp {
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
        let length = LengthOp(Box::new(NoOp::new_var()));

        Self {
            root: Box::new(length)
        }
    }

    pub fn circle(center: &Vec3, radius: f32) -> Self {
        // length(P-C)-r, where P is query point, C is Center vec and r is radius
        let center_neg = NegOp(Box::new(NoOp::new_const(&VecType::Vec3(*center))));
        let center_var_sub = AddOp {
            lhs: Box::new(NoOp::new_var()),
            rhs: Box::new(center_neg),
        };
        let dist_from_center = LengthOp(Box::new(center_var_sub));
        let radius_neg = NegOp(Box::new(NoOp::new_const(&VecType::Vec1(Vec1::new(radius)))));
        let radius_sub = AddOp {
            lhs: Box::new(dist_from_center),
            rhs: Box::new(radius_neg)
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
