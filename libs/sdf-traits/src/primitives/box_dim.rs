use sdf_vecs::{Vec1, Vec2, Vec3, VecType};

use crate::{Boxed, TraitSDF, ops::{Abs, Add, Length, Max, MaxComp, Min, NoOp, Sub}};

fn box_nd(dimensions: VecType) -> TraitSDF {
    // R is VecN(x, y, z, w)
    // P is point to sample at
    // d = length(max(abs(P) - R, 0)) -> single quadrant not interior
    //
    // q = abs(P) - R
    // d = length(max(q,0)) + min(max(q.x, q.y), 0)

    let abs_var = Abs::new(NoOp::new_var().boxed());
    let dim = NoOp::new_const(&dimensions);
    let zero = NoOp::new_const(&VecType::Vec1(Vec1::new(0.0)));

    // distance vector, will be re-used
    let d = Sub::new(
        abs_var.boxed(),
        dim.clone().boxed()
    );

    let max_zero = Max::new(
        d.clone().boxed(),
        zero.clone().boxed()
    );

    let length_max = Length::new(Box::new(max_zero));

    let dim_max_comp = MaxComp::new(Box::new(dim.clone()));

    let min_zero = Min::new(
        dim_max_comp.boxed(),
        zero.clone().boxed()
    );

    let root = Add::new(
        length_max.boxed(),
        min_zero.boxed()
    );

    TraitSDF::new(Box::new(root))
}

pub(crate) fn box_2d(width: f32, height: f32) -> TraitSDF {
    box_nd(VecType::Vec2(Vec2::new(width, height)))
}

pub(crate) fn box_3d(x: f32, y: f32, z: f32) -> TraitSDF {
    box_nd(VecType::Vec3(Vec3::new(x, y, z)))
}