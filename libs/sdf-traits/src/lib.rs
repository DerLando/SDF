use std::ops::DerefMut;

struct Vec2 {
    x: f32,
    y: f32
}

impl Vec2 {
    fn cross_product(&self, other: &Vec2) -> Vec2 {
        todo!()
    }
}

#[derive(Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    fn cross_product(&self, other: &Vec3) -> Vec3 {
        todo!()
    }

    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}

struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl Vec4 {
    fn cross_product(&self, other: &Vec4) -> Vec4 {
        todo!()
    }
}

/// All possible vec dimensions
enum Vec {
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4)
}

impl Vec {
    fn cross_product(a: &Vec, b: &Vec) -> Vec {
        match a {
            Vec::Vec2(va) => match b {
                Vec::Vec2(vb) => Vec::Vec2(va.cross_product(vb)),
                _ => unreachable!()
            },
            Vec::Vec3(va) => match b {
                Vec::Vec3(vb) => Vec::Vec3(va.cross_product(vb)),
                _ => unreachable!()
            },
            Vec::Vec4(va) => match b {
                Vec::Vec4(vb) => Vec::Vec4(va.cross_product(vb)),
                _ => unreachable!()
            },
        }
    }

    fn length(&self) -> f32 {
        todo!()
    }
}

/// A variable in a function
enum Variable {
    NumConst(Box<dyn Algebraic>),
    VecVar
}

enum VecVar {
    VecConst(Box<dyn Spatial>),
    Replaceable(Vec3)
}

/// After performing an operation, there can't be a replaceable variable left,
/// As it will have been consumed
enum EvaluatedVariable {
    VecConst(Vec),
    NumConst(f32)
}

impl std::ops::Add<EvaluatedVariable> for EvaluatedVariable {
    type Output = EvaluatedVariable;

    fn add(self, rhs: EvaluatedVariable) -> Self::Output {
        todo!()
    }
}

trait VariableContainer {
    fn replace_variable(&mut self, var: &Vec3);
}

/// The basic operator trait all operators have to implement
trait Operator<T> {
    fn operate(&self) -> T;
}

/// Operators that always return numbers
trait Algebraic: Operator<f32> + VariableContainer { }

/// Operators that always return a Vec
trait Spatial: Operator<Vec> + VariableContainer { }

/// Operators that have differing return types depending on their input
trait Mixed: Operator<EvaluatedVariable> + VariableContainer { }

struct Add {
    lhs: Variable,
    rhs: Variable
}

impl VariableContainer for Add {
    fn replace_variable(&mut self, var: &Vec3) {
        todo!()
    }
}

impl Mixed for Add { }

impl Operator<EvaluatedVariable> for Add {
    /// phew thats **a lot** of matching, but it makes sense, we want to support all those cases
    fn operate(&self) -> EvaluatedVariable {
        // match &self.lhs {
        //     Variable::NumConst(n_l) => match &self.rhs {
        //         Variable::NumConst(n_r) => EvaluatedVariable::NumConst(n_l.operate() + n_r.operate()),
        //         Variable::VecConst(v_r) => EvaluatedVariable::VecConst(n_l.operate() + v_r.operate()),
        //         Variable::Replaceable(r_r) => EvaluatedVariable::VecConst(n_l.operate() + r_r)
        //     },
        //     Variable::VecConst(v_l) => match &self.rhs {
        //         Variable::NumConst(n_r) => EvaluatedVariable::VecConst(v_l.operate() + n_r.operate()),
        //         Variable::VecConst(v_r) => EvaluatedVariable::VecConst(v_l.operate() + v_r.operate()),
        //         Variable::Replaceable(r_r) => EvaluatedVariable::VecConst(v_l.operate() + r_r)
        //     },
        //     Variable::Replaceable(r_l) => match &self.rhs {
        //         Variable::NumConst(n_r) => EvaluatedVariable::VecConst(r_l + n_r.operate()),
        //         Variable::VecConst(v_r) => EvaluatedVariable::VecConst(r_l + v_r.operate()),
        //         Variable::Replaceable(r_r) => EvaluatedVariable::VecConst(r_l + r_r)
        //     }
        // }
        todo!()
    }
}

struct Atan2 {
    lhs: Box<dyn Algebraic>,
    rhs: Box<dyn Algebraic>
}

impl VariableContainer for Atan2 {
    /// Actually makes no sense here, as atan2 never takes a vec arg
    fn replace_variable(&mut self, var: &Vec3) {
        todo!()
    }
}

impl Algebraic for Atan2 {}

impl Operator<f32> for Atan2 {
    fn operate(&self) -> f32 {
        self.lhs.operate().atan2(self.rhs.operate())
    }
}

struct CrossProduct {
    lhs: VecVar,
    rhs: VecVar,
}

impl VariableContainer for CrossProduct {
    fn replace_variable(&mut self, var: &Vec3) {
        todo!()
    }
}

impl Spatial for CrossProduct {}

impl Operator<Vec> for CrossProduct {
    fn operate(&self) -> Vec {
        todo!()
    }
}

struct NumConst(f32);

impl Algebraic for NumConst { }

impl VariableContainer for NumConst {
    fn replace_variable(&mut self, var: &Vec3) {
        todo!()
    }
}

impl Operator<f32> for NumConst {
    fn operate(&self) -> f32 {
        self.0
    }
}

struct Length(VecVar);

impl Algebraic for Length { }

impl VariableContainer for Length {
    fn replace_variable(&mut self, var: &Vec3) {
        match &mut self.0 {
            VecVar::Replaceable(v) => *v = *var,
            VecVar::VecConst(v) => v.replace_variable(var)
        }
    }
}

impl Operator<f32> for Length {
    fn operate(&self) -> f32 {
        match &self.0 {
            VecVar::VecConst(v) => v.operate().length(),
            VecVar::Replaceable(v) => v.length()
        }
    }
}

struct SDF {
    root: Box<dyn Algebraic>
}

impl SDF {
    pub fn sign_at(&mut self, position: &Vec3) -> f32 {
        // insert position for variable
        self.root.deref_mut().replace_variable(position);

        // operate the whole tree and return
        self.root.operate()

    }

    fn var_length() -> Self {
        let length = Length(VecVar::Replaceable(Vec3::default()));

        Self {
            root: Box::new(length)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_works() {
        let mut sdf = SDF::var_length();

        assert_eq!(1.0, sdf.sign_at(&Vec3{x: 1.0, y: 0.0, z: 0.0}));
        assert_eq!(1.0, sdf.sign_at(&Vec3{x: -1.0, y: 0.0, z: 0.0}));
        assert_eq!(2.0f32.sqrt(), sdf.sign_at(&Vec3{x: -1.0, y: 1.0, z: 0.0}));
    }
}
