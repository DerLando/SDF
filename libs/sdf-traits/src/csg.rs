use crate::{SDFTree, ops::{Constant, max, min, neg, sub, abs, div, mul}};

pub(crate) fn union(a: SDFTree, b: SDFTree) -> SDFTree {
    SDFTree::new(min(a, b))
}

pub(crate) fn difference(a: SDFTree, b: SDFTree) -> SDFTree {
    SDFTree::new(max(a, neg(b)))
}

pub(crate) fn intersection(a: SDFTree, b: SDFTree) -> SDFTree {
    SDFTree::new(max(a, b))
}

// cubic polynomial smoothing
fn smooth_min(a: SDFTree, b: SDFTree, k: f32) -> SDFTree {
    let k: Constant = k.into();
    let zero: Constant = 0.0.into();
    let one_sixth: Constant = (1.0 / 6.0).into();

    let h = div(max(sub(k.clone(), abs(sub(a.clone(), b.clone()))), zero), k.clone());
    let h_cube = mul(mul(h.clone(), h.clone()), h);
    SDFTree::new(sub(min(a, b), mul(mul(h_cube, k), one_sixth)))
}

pub(crate) fn union_smooth(a: SDFTree, b: SDFTree, k: f32) -> SDFTree {
    smooth_min(a, b, k)
}