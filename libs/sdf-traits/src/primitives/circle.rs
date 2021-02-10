use sdf_vecs::{Vec1, Vec3, VecType};

use crate::{TraitSDF, ops::{Add, Length, Neg, NoOp}};

pub(crate) fn circle(center: &Vec3, radius: f32) -> TraitSDF {
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

    TraitSDF::new(Box::new(radius_sub))
}
