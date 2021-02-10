use sdf_vecs::{Vec1, Vec3, VecType};

use crate::{Boxed, TraitSDF, ops::{Add, Length, Neg, NoOp}};

pub(crate) fn circle(center: &Vec3, radius: f32) -> TraitSDF {
    // length(P-C)-r, where P is query point, C is Center vec and r is radius
    let center_neg = Neg::new(NoOp::new_const(&VecType::Vec3(*center)).boxed());
    let center_var_sub = Add::new(
        NoOp::new_var().boxed(),
        center_neg.boxed()
    );

    let dist_from_center = Length::new(Box::new(center_var_sub));
    let radius_neg = Neg::new(NoOp::new_const(&VecType::Vec1(Vec1::new(radius))).boxed());
    let radius_sub = Add::new(
        dist_from_center.boxed(),
        radius_neg.boxed()
    );

    TraitSDF::new(radius_sub.boxed())
}
