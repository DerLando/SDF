use sdf_vecs::{Vec3};

use crate::{Spatial, ops::{length, sub, Constant, Variable,}};

pub(crate) fn circle(center: &Vec3, radius: f32) -> impl Spatial {
    // length(P-C)-r, where P is query point, C is Center vec and r is radius
    let center: Constant = (*center).into();
    let radius: Constant = radius.into();

    sub(length(sub(Variable, center)), radius)
}
