use criterion::{black_box, criterion_main, criterion_group, Criterion};
use glam::Vec3;
use sdf_traits::SDFTree;



fn circle_benchmark(c: &mut Criterion) {
    let mut circle = SDFTree::circle(&Vec3::default(), 10.0);
    circle.scale(1.618);
    let sample = Vec3::new(-0.43678, 7.118, -2.345);
    c.bench_function("circle_scale_benchmark", |b| b.iter(|| {
        circle.sign_at(&sample)
    }));
}

criterion_group!(benches, circle_benchmark);
criterion_main!(benches);