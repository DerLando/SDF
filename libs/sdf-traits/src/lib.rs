use sdf_vecs::{Vec3, Vec1, VecType, ops::{Length, add_high}, ComponentAccess};
use std::ops::DerefMut;

enum VecVar {
    Operator(Box<dyn Spatial>),
    Replaceable(Vec3),
    Const(VecType)
}

impl VariableContainer for VecVar {
    fn replace_variable(&mut self, var: &Vec3) {
        match self {
            VecVar::Operator(o) => o.replace_variable(var),
            VecVar::Replaceable(r) => *r = *var,
            VecVar::Const(_) => ()
        }
    }
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

struct NoOp(VecVar);

impl VariableContainer for NoOp {
    fn replace_variable(&mut self, var: &Vec3) {
        self.0.replace_variable(var)
    }
}

impl Operator<VecType> for NoOp {
    fn operate(&self) -> VecType {
        match &self.0 {
            VecVar::Replaceable(r) => VecType::Vec3(*r),
            VecVar::Operator(v) => v.operate(),
            VecVar::Const(c) => *c
        }
    }
}

struct AddOp {
    lhs: VecVar,
    rhs: VecVar
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
        match &self.lhs {
            VecVar::Replaceable(v_l) => match &self.rhs {
                VecVar::Replaceable(v_r) => (*v_l + *v_r).into(),
                VecVar::Operator(v_r) => add_high(&VecType::Vec3(*v_l), &v_r.operate()),
                VecVar::Const(c) => add_high(&VecType::Vec3(*v_l), &c)
            },
            VecVar::Operator(o_l) => match &self.rhs {
                VecVar::Operator(o_r) => add_high(&o_r.operate(), &o_l.operate()),
                VecVar::Replaceable(v_r) => add_high(&o_l.operate(), &VecType::Vec3(*v_r)),
                VecVar::Const(c) => add_high(&o_l.operate(), &c)
            },
            VecVar::Const(c_l) => match &self.rhs {
                VecVar::Const(c_r) => add_high(&c_l, &c_r),
                VecVar::Operator(o) => add_high(&c_l, &o.operate()),
                VecVar::Replaceable(r) => add_high(&c_l, &VecType::Vec3(*r))
            }
        }
    }
}

struct LengthOp(VecVar);

impl Spatial for LengthOp { }

impl VariableContainer for LengthOp {
    fn replace_variable(&mut self, var: &Vec3) {
        self.0.replace_variable(var);
    }
}

impl Operator<VecType> for LengthOp {
    fn operate(&self) -> VecType {
        match &self.0 {
            VecVar::Replaceable(r) => Vec1::new(r.length()).into(),
            VecVar::Operator(o) => Vec1::new(o.operate().length()).into(),
            VecVar::Const(c) => Vec1::new(c.length()).into()
        }
    }
}

struct NegOp(VecVar);

impl Spatial for NegOp { }

impl VariableContainer for NegOp {
    fn replace_variable(&mut self, var: &Vec3) {
        self.0.replace_variable(var)
    }
}

impl Operator<VecType> for NegOp {
    fn operate(&self) -> VecType {
        match &self.0 {
            VecVar::Operator(v) => -v.operate(),
            VecVar::Replaceable(v) => VecType::Vec3(-*v),
            VecVar::Const(c) => -*c
        }
    }
}

struct TraitSDF {
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
        let length = LengthOp(VecVar::Replaceable(Vec3::default()));

        Self {
            root: Box::new(length)
        }
    }

    fn circle(center: &Vec3, radius: f32) -> Self {
        // length(P-C)-r, where P is query point, C is Center vec and r is radius
        let center_neg = NegOp(VecVar::Const(VecType::Vec3(*center)));
        let center_var_sub = AddOp {
            lhs: VecVar::Replaceable(Vec3::default()),
            rhs: VecVar::Operator(Box::new(center_neg))
        };
        let dist_from_center = LengthOp(VecVar::Operator(Box::new(center_var_sub)));
        let radius_neg = NegOp(VecVar::Const(Vec1::new(radius).into()));
        let radius_sub = AddOp {
            lhs: VecVar::Operator(Box::new(dist_from_center)),
            rhs: VecVar::Operator(Box::new(radius_neg))
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
