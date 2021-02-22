use criterion::{black_box, criterion_main, criterion_group, Criterion};
use sdf_nodes::SdfTree;
use sdf_vecs::Vec3;



fn circle_benchmark(c: &mut Criterion) {
    let circle = SdfTree::circle(10.0);
    let sample = Vec3::new(-0.43678, 7.118, -2.345);
    c.bench_function("circle_node_100", |b| b.iter(|| {
        circle.sign_at(&sample)
    }));
}

criterion_group!(benches, circle_benchmark);
criterion_main!(benches);