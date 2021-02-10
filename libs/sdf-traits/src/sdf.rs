use std::ops::DerefMut;

use sdf_vecs::{ComponentAccess, Vec1, Vec3, VecType};

use crate::{Spatial, ops::{Add, Length, Neg, NoOp}};

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
        let length = Length(NoOp::new_var());

        Self {
            root: Box::new(length)
        }
    }

    pub fn circle(center: &Vec3, radius: f32) -> Self {
        // length(P-C)-r, where P is query point, C is Center vec and r is radius
        let center_neg = Neg(NoOp::new_const(&VecType::Vec3(*center)));
        let center_var_sub = Add {
            lhs: NoOp::new_var(),
            rhs: center_neg,
        };
        let dist_from_center = Length(center_var_sub);
        let radius_neg = Neg(NoOp::new_const(&VecType::Vec1(Vec1::new(radius))));
        let radius_sub = Add {
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
