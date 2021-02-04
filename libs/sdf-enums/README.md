# Enum based SDF

This will be the *mvp* for enum-based implementation of *Signed distance fields*

# Design

We compose operations as enums that store their values to operate on.

## Operators

Basically all operators will be stored in a big enum. Their arguments will be stored on the enum variants itself indirectly.

For this to work *flexibly*, we need *variable* enums, too:

 - `VecVar`, either a `Vec` or a `Replacable(`Vec3`)`,
 - `NumConst`, an f32

I'm not sure right now how to make this tree-like, though.

```rust
/// a number, or something that evaluates to a number
enum Number {
	Constant(f32),
	// This is not type-safe, we need to enforce numbers via more fine-grained enums
	// NumberReturnOperator, VecReturnOperator, MixedReturnOperator
	Operator(Operator)
}

/// a constant vec, a replacable vec, or something that evaluates to a vec
enum Vec {
	Constant(Vec3),
	Replacable(Vec3),
	Operator(Operator)
}

/// just give me anything, I don't care
enum Variable {
	Number(Number),
	Vec(Vec)
}

enum Operator {
	Add(Variable, Variable),
	Clamp(Variable, Number, Number),
	Length(Vec)
}

```