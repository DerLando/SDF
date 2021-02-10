use sdf_vecs::{Vec1, Vec3, VecType};

use crate::{TraitSDF, ops::{Add, Length, Neg, NoOp}};

pub(crate) fn circle(center: &Vec3, radius: f32) -> TraitSDF {
    // length(P-C)-r, where P is query point, C is Center vec and r is radius
    let center_neg = Neg::new(Box::new(NoOp::new_const(&VecType::Vec3(*center))));
    let center_var_sub = Add::new(
        Box::new(NoOp::new_var()),
        Box::new(center_neg)
    );

    let dist_from_center = Length::new(Box::new(center_var_sub));
    let radius_neg = Neg::new(Box::new(NoOp::new_const(&VecType::Vec1(Vec1::new(radius)))));
    let radius_sub = Add::new(
        Box::new(dist_from_center),
        Box::new(radius_neg)
    );

    TraitSDF::new(Box::new(radius_sub))
}
