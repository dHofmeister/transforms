use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use transforms::types::{Quaternion, Registry, Timestamp, Transform, Vector3};

#[cfg(feature = "async")]
use tokio::runtime::Runtime;

fn create_sample_transform() -> Transform {
    Transform {
        translation: Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: Quaternion {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        timestamp: Timestamp::now(),
        parent: "a".to_string(),
        child: "b".to_string(),
    }
}

#[cfg(not(feature = "async"))]
fn benchmark_sync_transforms(c: &mut Criterion) {
    let mut group = c.benchmark_group("sync_transforms");
    group.sample_size(1000);

    group.bench_function("sync_add_and_get_transform", |b| {
        let mut registry = Registry::new(Duration::from_secs(60));
        b.iter(|| {
            let transform = create_sample_transform();
            let _ = black_box(registry.add_transform(transform));
            let _ = black_box(registry.get_transform("a", "b", Timestamp::now()));
        });
    });

    group.finish();
}

#[cfg(feature = "async")]
fn benchmark_async_transforms(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_transforms");
    group.sample_size(1000);

    let rt = Runtime::new().unwrap();

    group.bench_function("async_add_and_get_transform", |b| {
        let registry = Registry::new(Duration::from_secs(60));
        b.iter(|| {
            rt.block_on(async {
                let transform = create_sample_transform();
                let _ = black_box(registry.add_transform(transform).await);
                let _ = black_box(registry.get_transform("a", "b", Timestamp::now()).await);
            });
        });
    });

    group.finish();
}

#[cfg(not(feature = "async"))]
criterion_group!(benches, benchmark_sync_transforms);
#[cfg(feature = "async")]
criterion_group!(benches, benchmark_async_transforms);
criterion_main!(benches);
