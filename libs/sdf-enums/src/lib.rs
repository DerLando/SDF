use sdf_vecs::{Vec3, Vec1, VecType, ops::{Length, add_high}, ComponentAccess};


/// a constant vec, a replacable vec, or something that evaluates to a vec
enum Variable {
	Constant(VecType),
	Replacable(Vec3),
	Operation(Box<Operation>)
}

impl Operator for Variable {
    fn operate(&self) -> Variable {
        match self {
            Variable::Constant(v) => Variable::Constant(*v),
            Variable::Replacable(v) => Variable::Constant(VecType::Vec3(*v)),
            Variable::Operation(o) => o.operate()
        }
    }
}

impl VariableContainer for Variable {
    fn replace_variable(&mut self, var: &Vec3) {
        match self {
            Variable::Replacable(r) => *r = *var,
            Variable::Operation(o) => o.replace_variable(var),
            Variable::Constant(_) => ()
        }    
    }
}

impl std::ops::Add<Variable> for Variable {
    type Output = Variable;

    fn add(self, rhs: Variable) -> Self::Output {
        match self {
            Variable::Constant(v_l) => match rhs {
                Variable::Constant(v_r) => Variable::Constant(add_high(&v_l, &v_r)),
                Variable::Replacable(r) => Variable::Constant(add_high(&v_l, &r.into())),
                Variable::Operation(o) => Variable::Constant(v_l) + o.operate()
            },
            Variable::Replacable(r_l) => match rhs {
                Variable::Constant(v_r) => Variable::Constant(add_high(&r_l.into(), &v_r)),
                Variable::Replacable(r) => Variable::Constant(add_high(&r_l.into(), &r.into())),
                Variable::Operation(o) => Variable::Constant(r_l.into()) + o.operate()
            },
            Variable::Operation(o_l) => match rhs {
                Variable::Constant(v_r) => Variable::Constant(v_r) + o_l.operate(),
                Variable::Replacable(r) => Variable::Constant(r.into()) + o_l.operate(),
                Variable::Operation(o) => o_l.operate() + o.operate()
            }
        }
    }
}

impl std::ops::Neg for Variable {
    type Output = Variable;

    fn neg(self) -> Self::Output {
        match self {
            Variable::Constant(v) => Variable::Constant(-v),
            Variable::Replacable(r) => Variable::Constant(VecType::Vec3(-r)),
            Variable::Operation(o) => -o.operate()
        }
    }
}

impl Variable {
    fn length(&self) -> f32 {
        match self {
            // recurse down until o.operate() returns a constant variant
            Variable::Operation(o) => o.operate().length(),
            Variable::Constant(v) => v.length(),
            Variable::Replacable(v) => v.length()
        }
    }
}

trait VariableContainer {
    fn replace_variable(&mut self, var: &Vec3);
}

trait Operator {
    fn operate(&self) -> Variable;
}

enum Operation {
    Cross(Variable, Variable),
    Neg(Variable),
    Add(Variable, Variable),
    Length(Variable)
}

impl Operator for Operation {
    fn operate(&self) -> Variable {
        match self {
            Operation::Neg(v) => -v.operate(),
            Operation::Add(vl, vr) => vl.operate() + vr.operate(),
            Operation::Cross(_, _) => unreachable!(),
            Operation::Length(v) => Variable::Constant(Vec1::new(v.length()).into())
        }
    }
}

impl VariableContainer for Operation {
    fn replace_variable(&mut self, var: &Vec3) {
        match self {
            Operation::Neg(v) => v.replace_variable(var),
            Operation::Add(vl, vr) => {
                vl.replace_variable(var);
                vr.replace_variable(var);
            },
            Operation::Cross(_, _) => unreachable!(),
            Operation::Length(v) => v.replace_variable(var)
        }
    }
}

struct EnumSDF {
    root: Operation
}

impl EnumSDF {
    pub fn sign_at(&mut self, position: &Vec3) -> f32 {
        // insert position for variable
        self.root.replace_variable(position);

        // operate the whole tree and return
        match self.root.operate() {
            Variable::Constant(v) => v.x(),
            _ => unreachable!()
        }
    }

    fn var_length() -> Self {
        let length = Operation::Length(Variable::Replacable(Vec3::default()));

        Self {
            root: length
        }
    }

    pub fn circle(center: &Vec3, radius: f32) -> Self {
        // length(P-C)-r, where P is query point, C is Center vec and r is radius
        let center_neg = Operation::Neg(Variable::Constant(VecType::Vec3(*center)));
        let center_var_sub = Operation::Add(
            Variable::Replacable(Vec3::default()),
            Variable::Operation(Box::new(center_neg))
        );
        let dist_from_center = Operation::Length(
            Variable::Operation(Box::new(center_var_sub))
        );
        let radius_neg = Operation::Neg(Variable::Constant(Vec1::new(radius).into()));
        let radius_sub = Operation::Add(
            Variable::Operation(Box::new(dist_from_center)),
            Variable::Operation(Box::new(radius_neg))
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

        assert_eq!(1.0, sdf.sign_at(&Vec3::new(1.0, 0.0, 0.0)));
        assert_eq!(1.0, sdf.sign_at(&Vec3::new(-1.0, 0.0, 0.0)));
        assert_eq!(2.0f32.sqrt(), sdf.sign_at(&Vec3::new(-1.0, 1.0, 0.0)));
    }

    #[test]
    fn circle_works() {
        let mut sdf = EnumSDF::circle(&Vec3::new(0.0, -1.0, 0.0), 10.0);

        assert_eq!(-10.0, sdf.sign_at(&Vec3::new(0.0, -1.0, 0.0)));
        assert_eq!(0.0, sdf.sign_at(&Vec3::new(10.0, -1.0, 0.0)));
        assert_eq!(10.0, sdf.sign_at(&Vec3::new(20.0, -1.0, 0.0)));
    }
}