use crate::{Spatial, ops::{Constant, max, min, neg, sub, abs, div, mul}};

pub(crate) fn union(a: impl Spatial + 'static, b: impl Spatial + 'static) -> impl Spatial {
    min(a, b)
}

pub(crate) fn difference(a: impl Spatial + 'static, b: impl Spatial + 'static) -> impl Spatial {
    max(a, neg(b))
}

pub(crate) fn intersection(a: impl Spatial + 'static, b: impl Spatial + 'static) -> impl Spatial {
    max(a, b)
}

// cubic polynomial smoothing
fn smooth_min(a: impl Spatial + 'static, b: impl Spatial + 'static, k: f32) -> impl Spatial {
    let k: Constant = k.into();
    let zero: Constant = 0.0.into();
    let one_sixth: Constant = (1.0 / 6.0).into();

    let h = div(max(sub(k, abs(sub(a.clone(), b.clone()))), zero), k);
    let h_cube = mul(mul(h.clone(), h.clone()), h);
    sub(min(a, b), mul(mul(h_cube, k), one_sixth))
}

pub(crate) fn union_smooth(a: impl Spatial + 'static, b: impl Spatial + 'static, k: f32) -> impl Spatial {
    smooth_min(a, b, k)
}