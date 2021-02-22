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

# Notes

While implementing `smooth_union` I recognized, that the current implementation makes it impossible to clone a `SDF` passed as a parameter `impl Spatial + 'static`. So we probably need to get back to a level of indirection where an `SDF` is explicitly wrapped inside of a `SDFTree` struct which implements clone. On the plus-side, with this approach we can overload operators and make library functions more readable / idiomatic.

I'm thinking about how to best implement Transformations. For now the easiest and most flexible choice is probably to store a Transformation Matrix on each `SDFTree` instance, but I am not sure how this will work when blending / merging multiple `SDFTree`s. Also to implement transformations we will now need to bring in a linear algebra crate.

A low-hanging fruit for optimization is evaluating all constant expressions inside a `SDFTree`, folding it into only the variable parts. This decreases the size and the number of operations that have to be performed. F.e. v + 2 + 3 should simplify to v + 5. For this we will need a TreeWalker that can traverse up the tree, folding all simplifyable nodes it finds. As all we know about the nodes are `dyn Spatial`, we need to add a trait to expose the inner components of the ops to manipulate them.
Some initial invastigation hints that we have to implement `as_any` for `Spatial` and use *runtime reflection* to get the concrete types in our optimization pass. This should be fine, as we only ever do this once, when a `TreeSDF`s sign is queried for the first time. We should probably also expose this simplification as a public method, as in a parallelized environment, one would want to call this before openening all threads.
The example mentioned here: https://doc.rust-lang.org/std/any/ , is pretty close to our use case.
Although matching for all different op types sounds nightmarish :/

Another way would be a `real` Tree which has a backing `HashSet<Constant>` and hands out references to the constants to all operators. On paper this sounds like a serious departure from Trait opjects, so it would need to be prototyped in a different crate. Probably a good time to finally implement a node-based tree representation of sdf.

# TODO

 - [x] allow sampling of `sdf` without mutable access, this involves multiple steps:
   - [x] change the signature of the `operate` function to `operate(&self, pos: &Vec3)`
   - [x] get rid of `VariableContainer` trait
   - [x] implement a new Operator, `VarOp` which will evaluate to whatever variable given.
 - [x] add layer of indirection via `SDFTree` wrapper struct