use common::cache::{Cache, CacheExt, LocalCache};
use common::config::CacheSettings;
use criterion::{Criterion, criterion_group, criterion_main};
use std::time::Duration;

fn make_cache() -> LocalCache {
    LocalCache::new(&CacheSettings {
        max_capacity: 10_000,
        ttl_secs: 300,
        tti_secs: 60,
        redis: None,
    })
}

fn bench_cache_set(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = make_cache();

    c.bench_function("local_cache_set", |b| {
        b.to_async(&rt).iter(|| async {
            cache
                .set_str("bench_key", "bench_value", Duration::from_secs(300))
                .await;
        });
    });
}

fn bench_cache_get_hit(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = make_cache();

    rt.block_on(async {
        cache
            .set_str("hit_key", "hit_value", Duration::from_secs(300))
            .await;
    });

    c.bench_function("local_cache_get_hit", |b| {
        b.to_async(&rt).iter(|| async {
            cache.get_str("hit_key").await;
        });
    });
}

fn bench_cache_get_miss(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = make_cache();

    c.bench_function("local_cache_get_miss", |b| {
        b.to_async(&rt).iter(|| async {
            cache.get_str("nonexistent_key").await;
        });
    });
}

fn bench_cache_delete(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = make_cache();

    c.bench_function("local_cache_delete", |b| {
        b.to_async(&rt).iter_custom(|iters| {
            let cache = &cache;
            async move {
                for i in 0..iters {
                    cache
                        .set_str(&format!("del_key_{i}"), "value", Duration::from_secs(300))
                        .await;
                }

                let start = std::time::Instant::now();
                for i in 0..iters {
                    cache.delete_str(&format!("del_key_{i}")).await;
                }
                start.elapsed()
            }
        });
    });
}

fn bench_cache_exists(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = make_cache();

    rt.block_on(async {
        cache
            .set_str("exists_key", "value", Duration::from_secs(300))
            .await;
    });

    c.bench_function("local_cache_exists_hit", |b| {
        b.to_async(&rt).iter(|| async {
            cache.exists_str("exists_key").await;
        });
    });

    c.bench_function("local_cache_exists_miss", |b| {
        b.to_async(&rt).iter(|| async {
            cache.exists_str("no_key").await;
        });
    });
}

fn bench_cache_json_roundtrip(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = make_cache();

    c.bench_function("local_cache_json_set", |b| {
        b.to_async(&rt).iter(|| async {
            let payload = serde_json::json!({
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "title": "Benchmark Post",
                "content": "Some benchmark content that is reasonably sized for a blog post.",
            });
            cache
                .set("json_key", payload, Duration::from_secs(300))
                .await;
        });
    });

    rt.block_on(async {
        let payload = serde_json::json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "title": "Benchmark Post",
            "content": "Some benchmark content that is reasonably sized for a blog post.",
        });
        cache
            .set("json_key", payload, Duration::from_secs(300))
            .await;
    });

    c.bench_function("local_cache_json_get", |b| {
        b.to_async(&rt).iter(|| async {
            let _: Option<serde_json::Value> = cache.get("json_key").await;
        });
    });
}

fn bench_cache_throughput(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = make_cache();

    let mut group = c.benchmark_group("local_cache_mixed_workload");

    group.bench_function("set_then_get_100", |b| {
        b.to_async(&rt).iter(|| async {
            for i in 0..100 {
                let key = format!("throughput_{i}");
                cache.set_str(&key, "value", Duration::from_secs(300)).await;
                cache.get_str(&key).await;
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_cache_set,
    bench_cache_get_hit,
    bench_cache_get_miss,
    bench_cache_delete,
    bench_cache_exists,
    bench_cache_json_roundtrip,
    bench_cache_throughput,
);
criterion_main!(benches);
