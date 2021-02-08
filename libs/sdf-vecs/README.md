# Vecs

Abstraction and general purpose methods for vecs used in `sdf` computations

# Dimensions

Vecs up to 4 dimensions are supported. The component type is f32:

 - Vec1
 - Vec2
 - Vec3
 - Vec4

# Methods

Methods generally come in 2 flavors:

 - `xx_high` -> *Upscales all Vecs to the highest dimensional Vec*
 - `xx_low` -> *Downscales all Vecs to the lowest dimensional Vec*

## Scaling

### Up

When scaling the dimension of a vec up, it's fields get repeated, until the desired dimension is reached.

```rust

let pt = Vec2::new(2.0, -1.0);
let scaled = pt.scale_4();
assert_eq!(Vec4::new(2.0, -1.0, 2.0, -1.0), scaled);

```

### Down

When scaling the dimension of a vec down, the fields of the higher dimensions get omitted.

```rust

let pt = Vec3::new(3.0, 2.0, -1.0);
let scaled = pt.scale_2();
assert_eq!(Vec2::new(3.0, 2.0), scaled);

```