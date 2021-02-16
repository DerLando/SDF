use glam::Vec2;

use crate::{Spatial, ops::{Constant, Variable, vec2, sub, length, xz, x, y}};

pub(crate) fn torus(inner_radius: f32, outer_radius: f32) -> impl Spatial {
    // q = vec2(length(p.xz)-t.x,p.y)
    // distance = length(q)-t.y
    let p = Variable;
    let t: Constant = Vec2::new(inner_radius, outer_radius).into();
    let q = vec2(sub(length(xz(p.clone())), x(t.clone())), y(p.clone()));

    sub(length(q), y(t))
}