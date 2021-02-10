use sdf_vecs::{Vec1, Vec2, Vec3, VecType};

use crate::{TraitSDF, ops::{Abs, Add, Length, Max, MaxComp, Min, NoOp, Sub}};

fn box_nd(dimensions: VecType) -> TraitSDF {
    // R is VecN(x, y, z, w)
    // P is point to sample at
    // d = length(max(abs(P) - R, 0)) -> single quadrant not interior
    //
    // q = abs(P) - R
    // d = length(max(q,0)) + min(max(q.x, q.y), 0)

    let abs_var = Abs(NoOp::new_var());
    let dim = NoOp::new_const(&dimensions);
    let zero = NoOp::new_const(&VecType::Vec1(Vec1::new(0.0)));

    // distance vector, will be re-used
    let d = Sub {
        lhs: abs_var,
        rhs: dim.clone()
    };

    let max_zero = Max {
        lhs: d.clone(),
        rhs: zero.clone()
    };

    let length_max = Length(max_zero);

    let dim_max_comp = MaxComp(dim.clone());

    let min_zero = Min {
        lhs: dim_max_comp,
        rhs: zero.clone()
    };

    let root = Add {
        lhs: length_max,
        rhs: min_zero
    };

    TraitSDF::new(Box::new(root))
}

pub(crate) fn box_2d(width: f32, height: f32) -> TraitSDF {
    box_nd(VecType::Vec2(Vec2::new(width, height)))
}

pub(crate) fn box_3d(x: f32, y: f32, z: f32) -> TraitSDF {
    box_nd(VecType::Vec3(Vec3::new(x, y, z)))
}