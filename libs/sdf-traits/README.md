# Trait-based SDF

This is the *mvp* for a trait-based implementation of *Signed distance fields*.

# Design

We compose *operations* on *values* as trait objects.

## Operators

Operators have to define multiple traits, to be recognized as such. It is recommended to encode the trait objects as `struct-generics`:

```rust

struct NegOp<S: Sized>(S)

impl<S> Spatial for NegOp<S> where S: Spatial { }

```

The main traits are:
 - `Operator<T>`, which gives the `operate(&self) -> T` method. All other operator traits derive from this
 - `VariableContainer`, which gives the `replace_variable(&mut self)` method. This replaces every occurence of the *Position* variable with the given position.
 - `Spatial`, which implements `Operator<Vec>`. All operators that always return a `Vec`, f.e. *YX*, *Cross*, or *Vec4*

This is **very** flexible and allows us to precisely model the very different arguments needed for different operations.
*Clamp*, *Swizzling*, *Atan2* and *Length* f.e. have vastly differing input requirements and we need to be able to abstract over all of them.

## Sign

To be able to dynamically evaluate the sign of any given position in space, the `NoOp` operator can store a `Replacable(Vec3)` enum Variant and implements the `VariableContainer` trait.

The variants to describe dynamic variables inside `NoOp` are:
 - `Const(VecType)`, if your argument will only ever evaluate to a `VecType`
 - `Replaceable(Vec3)`, which will be replaced, with the position at which the *SDF* is sampled

## SDF interface

Users can interface via the public `TraitSDF` struct, which exposes the `sign_at(&mut self, position: &Vec3)` method. It is implemented like so:

```rust
struct SDF {
    // by forcing root to be an `Algebraic` trait object, we know for certain that it will evaluate to an f32
    root: Box<dyn Algebraic>
}
```

When calling the `TraitSDF::sign_at(&mut self, position: &Vec3)` function, the SDF will first traverse down it's operation tree, replacing all occurences of `Replacable(Vec3)` with the given position. Afterwards it will call it's roots `operate` method, which will recursively evaluate all operations and ultimately return the correct sign.

While mutable access to the `SDF` is quite unflexible, it could only be circumvented by cloning the whole `SDF` on every call to `sign_at` and manipulating the copy. As an `SDF` is typically evaluated for pixels or voxels, this would mean **millions** of additional allocations.

## Cons

Since we are passing around trait objects, everything is *boxed* and stored on the heap. This probably won't be too nice for performance overall, we will need to benchmark this against other implementations.

# TODO

 - [x] allow sampling of `sdf` without mutable access, this involves multiple steps:
   - [x] change the signature of the `operate` function to `operate(&self, pos: &Vec3)`
   - [x] get rid of `VariableContainer` trait
   - [x] implement a new Operator, `VarOp` which will evaluate to whatever variable given.