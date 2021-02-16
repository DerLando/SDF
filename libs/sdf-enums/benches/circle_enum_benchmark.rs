use criterion::{black_box, criterion_main, criterion_group, Criterion};
use glam::Vec3;
use sdf_enums::EnumSDF;



fn circle_benchmark(c: &mut Criterion) {
    let mut circle = EnumSDF::circle(&Vec3::default(), 10.0);
    let sample = Vec3::new(-0.43678, 7.118, -2.345);
    c.bench_function("circle_enum_100", |b| b.iter(|| {
        circle.sign_at(&sample);
    }));
}

criterion_group!(benches, circle_benchmark);
criterion_main!(benches);