use sdf_vecs::{Vec1, Vec3, VecType};

use crate::{Spatial, TraitSDF, ops::{add, length, neg, sub, NoOp,}};

pub(crate) fn circle(center: &Vec3, radius: f32) -> impl Spatial {
    // length(P-C)-r, where P is query point, C is Center vec and r is radius
    let center: NoOp = (*center).into();
    let radius: NoOp = radius.into();

    sub(length(sub(NoOp::new_var(), center)), radius)
}
