# Enum based SDF

This will be the *mvp* for enum-based implementation of *Signed distance fields*

# Design

We compose operations as enums that store their values to operate on.

## Operators

Basically all operators will be stored in a big enum. Their arguments will be stored on the enum variants itself indirectly.

For this to work *flexibly*, we need *variable* enums, too:

 - `VecVar`, either a `Vec` or a `Replacable(`Vec3`)`,
 - `NumConst`, an f32

For now, operators are ordered by their return type, which gives 3 enums instead of one. I can't think of a good way to construct them from a big enum.

```rust

enum Operator {
	Length,
	Add,
	Clamp
}

impl Operator {
	// How can we give a varying number of args here?
	fn create(self) -> T // how can we return the correct type?
}

```