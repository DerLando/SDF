use sdf_vecs::{Vec1, Vec2, Vec3, VecType};

use crate::{Spatial, ops::{abs, add, length, max, max_comp, min, Constant, Variable, sub}};

fn box_nd(dimensions: VecType) -> impl Spatial {
    // R is VecN(x, y, z, w)
    // P is point to sample at
    // d = length(max(abs(P) - R, 0)) -> single quadrant not interior
    //
    // q = abs(P) - R
    // d = length(max(q,0)) + min(max(q.x, q.y), 0)

    let dim = Constant(dimensions);
    let zero: Constant = 0.0.into();

    let q = sub(abs(Variable), dim);
    let lhs = length(max(q.clone(), zero.clone()));
    let rhs = min(max_comp(q), zero);

    add(lhs, rhs)
}

pub(crate) fn box_2d(width: f32, height: f32) -> impl Spatial {
    box_nd(VecType::Vec2(Vec2::new(width, height)))
}

pub(crate) fn box_3d(x: f32, y: f32, z: f32) -> impl Spatial {
    box_nd(VecType::Vec3(Vec3::new(x, y, z)))
}