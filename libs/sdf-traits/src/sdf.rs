use sdf_vecs::{RotationAxis, Transform, TransformHelper, Vec3, VecType};

use crate::{Operator, Spatial, csg::{difference, intersection, union, union_smooth}, ops::{Constant, Variable, add, length, max, min, mul, sub}, primitives::{box_2d, box_3d, circle, torus}};

#[derive(Clone)]
pub struct SDFTree {
    pub(crate) root: Box<dyn Spatial>,
    transform: Transform,
    scale_fac: f32
}

impl Operator for SDFTree {
    fn operate(&self, pos: &Vec3) -> VecType {
        let mut p: Vec3 = *pos;

        // test if we need to compress space
        if self.need_compress_space() {p = p / self.scale_fac};

        // apply transformation to position
        p = self.transform.transform_point3(p);
        
        self.root.operate(&p)
    }
}

impl Spatial for SDFTree {}

impl SDFTree {
    pub(crate) fn new(sdf: impl Spatial + 'static) -> Self {
        Self {
            root: Box::new(sdf),
            transform: Transform::identity(),
            scale_fac: 1.0
        }
    }

    pub fn sign_at(&self, position: &Vec3) -> f32 {
        // operate the whole tree and return
        match self.operate(position) {
            VecType::Scalar(s) => s,
            _ => unreachable!()
        }

    }
}

/// Primitives
impl SDFTree {
    pub fn point() -> Self {
        Self::new(length(Variable))
    }

    pub fn circle(center: &Vec3, radius: f32) -> Self {
        // length(P-C)-r, where P is query point, C is Center vec and r is radius
        Self::new(circle(center, radius))
    }

    pub fn sphere(radius: f32) -> Self {
        let r: Constant = radius.into();
        Self::new(sub(length(Variable), r))
    }

    /// Create a rectangle centered at (0, 0), with max extents of x and y
    pub fn rectangle(width: f32, height: f32) -> Self {
        Self::new(box_2d(width, height))
    }

    pub fn cuboid(x: f32, y: f32, z: f32) -> Self {
        Self::new(box_3d(x, y, z))
    }

    pub fn torus(inner_radius: f32, outer_radius: f32) -> Self {
        Self::new(torus(inner_radius, outer_radius))
    }
}

/// CSG
impl SDFTree {
    pub fn union(a: SDFTree, b: SDFTree) -> Self {
        union(a, b)
    }

    pub fn intersection(a: SDFTree, b: SDFTree) -> Self {
        intersection(a, b)
    }

    pub fn difference(a: SDFTree, b: SDFTree) -> Self {
        difference(a, b)
    }

    pub fn union_smooth(a: SDFTree, b: SDFTree, k: f32) -> Self {
        union_smooth(a, b, k)
    }
}

/// Transforms
impl SDFTree {
    pub fn translate(&mut self, v: &Vec3) {
        // apply inverse of translation AFTER the original transformation
        self.transform = TransformHelper::translation(v).inverse() * self.transform;
    }

    pub fn rotate(&mut self, angle: f32, axis: RotationAxis) {
        self.transform = TransformHelper::rotate(angle, axis).inverse() * self.transform;
    }

    pub fn scale(&mut self, scale_factor: f32) {
        // update scale_fac
        self.scale_fac *= scale_factor;

        // wrap root in mul op
        let s: Constant = scale_factor.into();
        self.root = Box::new(mul(self.root.clone(), s));
    }

    fn need_compress_space(&self) -> bool {
        self.scale_fac != 1.0
    }
}

/// General purpose
impl SDFTree {
    pub fn blend(a: SDFTree, b: SDFTree, factor: f32) -> impl Spatial {
        // a is blend factor, L is left tree and R is right tree
        // d = (1 - a) * L + a * R

        let factor: Constant = factor.into();
        let one: Constant = 1.0.into();

        let lhs = mul(sub(one, factor.clone()), a);
        let rhs = mul(factor, b);

        add(lhs, rhs)
    }

    pub fn rounded_edges(sdf: SDFTree, radius: f32) -> impl Spatial {
        let r: Constant = radius.into();
        sub(sdf, r)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_works() {
        let mut sdf = SDFTree::circle(&Vec3::new(0.0, -1.0, 0.0), 10.0);

        assert_eq!(-10.0, SDFTree::sign_at(&mut sdf, &Vec3::new(0.0, -1.0, 0.0)));
        assert_eq!(0.0, SDFTree::sign_at(&mut sdf, &Vec3::new(10.0, -1.0, 0.0)));
        assert_eq!(10.0, SDFTree::sign_at(&mut sdf, &Vec3::new(20.0, -1.0, 0.0)));
    }

    // #[test]
    // fn rectangle_works() {
    //     let mut rect = SDFTree::rectangle(3.0, 6.0);

    //     assert_eq!(0.0, rect.sign_at(&Vec3::new(3.0, 6.0, 0.0)));
    //     assert_eq!(1.5, rect.sign_at(&Vec3::new(4.5, 3.0, 0.0)));
    // }

    // #[test]
    // fn cuboid_works() {
    //     let mut cuboid = SDFTree::cuboid(1.0, 2.0, 3.0);

    //     assert_eq!(0.0, cuboid.sign_at(&Vec3::new(1.0, 2.0, 3.0)));
    //     assert_eq!(2.0, cuboid.sign_at(&Vec3::new(3.0, 0.0, 0.0)));
    // }
}
