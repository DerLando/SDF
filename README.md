# SDF-rs

Implementation of **S**igned **D**istance **F**ields data structures and algorithms.

The main things this library will achieve *in the future* are:
 - Generate a `SDF`
 - Manipulate or combine multiple `SDF`
 - Generate basic triangle meshes from a given `SDF`

We want to implement multiple API surfaces:
 - Use as a rust library
 - FFI via function calls
 - Fluent `SDF` creation via an interpreted functional-style language

 # MVPs

 As a first step I will be implementing multiple MVPs which differ in their internal representation of a `SDF`:

  - `sdf-enums`, which represents a `SDF` as nested enums
  - `sdf-nodes`, which represents a `SDF` as a tree of nodes
  - `sdf-traits`, which represents a `SDF` as nested operators via trait objects

## Feature set

The MVP implementations need to implement a minimum feature set, so we can bench them against each other:

 - `circle` primitive
 - `transformations`, *scale* as a factor and *affine* via a transformation matrix
 - `simplify`, combines all constant expressions, f.e. 1 + 1 just becomes 2. *This is not too critical* 