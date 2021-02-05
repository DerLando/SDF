#[derive(Clone, Copy, Default)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl std::ops::Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        self + Vec3{x: rhs, y: rhs, z: rhs}
    }
}

/// a number, or something that evaluates to a number
enum Number {
	Constant(f32),
	Operator(Box<NumberReturningOperator>)
}

impl Operator<f32> for Number {
    fn operate(&self) -> f32 {
        match self {
            Number::Operator(o) => o.operate().operate(),
            Number::Constant(n) => *n
        }
    }
}

impl VariableContainer for Number {
    fn replace_variable(&mut self, var: &Vec3) {
        match self {
            Number::Operator(o) => o.replace_variable(var),
            Number::Constant(_) => ()
        }
    }
}

impl std::ops::Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Number::Constant(self.operate() + rhs.operate())
    }
}

/// a constant vec, a replacable vec, or something that evaluates to a vec
enum Vec {
	Constant(Vec3),
	Replacable(Vec3),
	Operator(Box<VecReturningOperator>)
}

impl VariableContainer for Vec {
    fn replace_variable(&mut self, var: &Vec3) {
        match self {
            Vec::Replacable(r) => *r = *var,
            Vec::Operator(o) => o.replace_variable(var),
            Vec::Constant(_) => ()
        }    
    }
}

impl Vec {
    fn length(&self) -> f32 {
        match self {
            // recurse down until o.operate() returns a constant variant
            Vec::Operator(o) => o.operate().length(),
            Vec::Constant(v) | Vec::Replacable(v) => v.length()
        }
    }
}

impl Operator<Vec3> for Vec {
    fn operate(&self) -> Vec3 {
        match self {
            Vec::Operator(o) => o.operate().operate(),
            Vec::Constant(v) | Vec::Replacable(v) => *v
        }
    }
}

impl std::ops::Add<Vec> for Vec {
    type Output = Vec;

    fn add(self, rhs: Vec) -> Self::Output {
        Vec::Constant(self.operate() + rhs.operate())
    }
}

impl std::ops::Neg for Vec {
    type Output = Vec;

    fn neg(self) -> Self::Output {
        Vec::Constant(-self.operate())
    }
}

/// just give me anything, I don't care
enum Variable {
	Number(Number),
	Vec(Vec)
}

impl Variable {
    fn as_num(self) -> Number {
        match self {
            Variable::Number(n) => n,
            Variable::Vec(v) => Number::Constant(v.length())
        }
    }

    fn as_vec(self) -> Vec {
        match self {
            Variable::Vec(v) => v,
            Variable::Number(n) => Vec::Constant(Vec3{x: n.operate(), y: n.operate(), z: n.operate()})
        }
    }
}

impl std::ops::Add<Variable> for Variable {
    type Output = Variable;

    fn add(self, rhs: Variable) -> Self::Output {
        match self {
            Variable::Number(num_l) => match rhs {
                Variable::Number(num_r) => Variable::Number(num_l + num_r),
                Variable::Vec(v) => Variable::Vec(Vec::Constant(v.operate() + num_l.operate()))
            },
            Variable::Vec(v_l) => match rhs {
                Variable::Number(n) => Variable::Vec(Vec::Constant(v_l.operate() + n.operate())),
                Variable::Vec(v_r) => Variable::Vec(v_l + v_r)
            }
        }
    }
}

impl std::ops::Neg for Variable {
    type Output = Variable;

    fn neg(self) -> Self::Output {
        match self {
            Variable::Number(n) => Variable::Number(Number::Constant(-n.operate())),
            Variable::Vec(v) => Variable::Vec(Vec::Constant(-v.operate()))
        }
    }
}

impl VariableContainer for Variable {
    fn replace_variable(&mut self, var: &Vec3) {
        match self {
            Variable::Number(n) => n.replace_variable(var),
            Variable::Vec(v) => v.replace_variable(var)
        }
    }
}

trait Operator<T> {
    fn operate(&self) -> T;
}

trait VariableContainer {
    fn replace_variable(&mut self, var: &Vec3);
}

enum NumberReturningOperator {
	Length(Vec),
    Neg(Number),
    Add(Number, Number)
}

impl Operator<Number> for NumberReturningOperator {
    fn operate(&self) -> Number {
        match self {
            NumberReturningOperator::Length(v) => Number::Constant(v.length()),
            NumberReturningOperator::Neg(n) => Number::Constant(-n.operate()),
            NumberReturningOperator::Add(n_l, n_r) => Number::Constant(n_l.operate() + n_r.operate())
        }
    }
}

impl VariableContainer for NumberReturningOperator {
    fn replace_variable(&mut self, var: &Vec3) {
        match self {
            NumberReturningOperator::Length(v) => v.replace_variable(var),
            NumberReturningOperator::Neg(n) => n.replace_variable(var),
            NumberReturningOperator::Add(n_l, n_r) => {
                n_l.replace_variable(var);
                n_r.replace_variable(var);
            }
        }
    }
}


enum VecReturningOperator {
    Cross(Vec, Vec),
    Neg(Vec),
    Add(Vec, Vec),
    AddNum(Vec, Number)
}

impl Operator<Vec> for VecReturningOperator {
    fn operate(&self) -> Vec {
        match self {
            VecReturningOperator::Neg(v) => Vec::Constant(-v.operate()),
            VecReturningOperator::Add(vl, vr) => Vec::Constant(vl.operate() + vr.operate()),
            VecReturningOperator::AddNum(v, n) => Vec::Constant(v.operate() + n.operate()),
            _ => unreachable!()
        }
    }
}

impl VariableContainer for VecReturningOperator {
    fn replace_variable(&mut self, var: &Vec3) {
        match self {
            VecReturningOperator::Neg(v) => v.replace_variable(var),
            VecReturningOperator::Add(vl, vr) => {
                vl.replace_variable(var);
                vr.replace_variable(var);
            },
            VecReturningOperator::AddNum(v, n) => {
                v.replace_variable(var);
                n.replace_variable(var);
            },
            VecReturningOperator::Cross(_, _) => unreachable!()
        }
    }
}

struct EnumSDF {
    root: NumberReturningOperator
}

impl EnumSDF {
    pub fn sign_at(&mut self, position: &Vec3) -> f32 {
        // insert position for variable
        self.root.replace_variable(position);

        // operate the whole tree and return
        match self.root.operate() {
            Number::Constant(n) => n,
            Number::Operator(_) => panic!()
        }
    }

    fn var_length() -> Self {
        let length = NumberReturningOperator::Length(Vec::Replacable(Vec3::default()));

        Self {
            root: length
        }
    }

    pub fn circle(center: &Vec3, radius: f32) -> Self {
        // length(P-C)-r, where P is query point, C is Center vec and r is radius
        let center_neg = VecReturningOperator::Neg(Vec::Constant(*center));
        let center_var_sub = VecReturningOperator::Add(
            Vec::Replacable(Vec3::default()),
            Vec::Operator(Box::new(center_neg))
        );
        let dist_from_center = NumberReturningOperator::Length(
            Vec::Operator(Box::new(center_var_sub))
        );
        let radius_neg = NumberReturningOperator::Neg(Number::Constant(radius));
        let radius_sub = NumberReturningOperator::Add(
            Number::Operator(Box::new(dist_from_center)),
            Number::Operator(Box::new(radius_neg))
        );

        Self {
            root: radius_sub
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_works() {
        let mut sdf = EnumSDF::var_length();

        assert_eq!(1.0, sdf.sign_at(&Vec3{x: 1.0, y: 0.0, z: 0.0}));
        assert_eq!(1.0, sdf.sign_at(&Vec3{x: -1.0, y: 0.0, z: 0.0}));
        assert_eq!(2.0f32.sqrt(), sdf.sign_at(&Vec3{x: -1.0, y: 1.0, z: 0.0}));
    }

    #[test]
    fn circle_works() {
        let mut sdf = EnumSDF::circle(&Vec3{x: 0.0, y: -1.0, z: 0.0}, 10.0);

        assert_eq!(-10.0, sdf.sign_at(&Vec3{x: 0.0, y: -1.0, z: 0.0}));
        assert_eq!(0.0, sdf.sign_at(&Vec3{x: 10.0, y: -1.0, z: 0.0}));
        assert_eq!(10.0, sdf.sign_at(&Vec3{x: 20.0, y: -1.0, z: 0.0}));
    }
}