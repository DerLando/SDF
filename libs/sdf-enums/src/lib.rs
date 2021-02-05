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

/// a number, or something that evaluates to a number
enum Number {
	Constant(f32),
	Operator(NumberReturningOperator)
}

impl VariableContainer for Number {
    fn replace_variable(&mut self, var: &Vec3) {
        match self {
            Number::Operator(o) => o.replace_variable(var),
            Number::Constant(_) => ()
        }
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

/// just give me anything, I don't care
enum Variable {
	Number(Number),
	Vec(Vec)
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
	Length(Vec)
}

impl Operator<Number> for NumberReturningOperator {
    fn operate(&self) -> Number {
        match self {
            NumberReturningOperator::Length(v) => Number::Constant(v.length())
        }
    }
}

impl VariableContainer for NumberReturningOperator {
    fn replace_variable(&mut self, var: &Vec3) {
        match self {
            NumberReturningOperator::Length(v) => v.replace_variable(var),
        }
    }
}


enum VecReturningOperator {
    Cross(Vec, Vec)
}

impl Operator<Vec> for VecReturningOperator {
    fn operate(&self) -> Vec {
        todo!()
    }
}

impl VariableContainer for VecReturningOperator {
    fn replace_variable(&mut self, var: &Vec3) {
        todo!()
    }
}

enum VariableReturningOperator {
	Add(Variable, Variable),
}

impl Operator<Variable> for VariableReturningOperator {
    fn operate(&self) -> Variable {
        todo!()
    }
}

struct SDF {
    root: NumberReturningOperator
}

impl SDF {
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