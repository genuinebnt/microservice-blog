use std::sync::Arc;

use chrono::Utc;
use common::cache::{LocalCache, RedisCache, TieredCache};
use common::config::CacheSettings;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use posts_service::domain::{Post, PostId, PostRepository};
use posts_service::infrastructure::database::seaorm::SeaOrmPostRepository;
use posts_service::infrastructure::database::types::DatabaseConn;
use posts_service::infrastructure::database::{CachedPostRepository, bootstrap};

async fn setup_db_repo() -> (Arc<SeaOrmPostRepository>, common::config::Settings) {
    unsafe { std::env::set_var("APP_ENVIRONMENT", "test") }

    let config = common::config::get_configuration::<common::config::Settings>("config").unwrap();
    let conn = bootstrap(&config.database).await.unwrap();

    match conn {
        DatabaseConn::SeaOrm(db) => {
            use migration::{Migrator, MigratorTrait};
            Migrator::up(&db, None).await.unwrap();
            (Arc::new(SeaOrmPostRepository::new(db)), config)
        }
        _ => panic!("expected SeaOrm backend"),
    }
}

fn make_post() -> Post {
    Post::builder()
        .id(PostId::new().into())
        .title("Bench Post".to_string())
        .author_id(uuid::Uuid::new_v4())
        .content("Benchmark content for testing cache performance".to_string())
        .created_at(Utc::now().into())
        .updated_at(Utc::now().into())
        .build()
}

fn cache_settings() -> CacheSettings {
    CacheSettings {
        max_capacity: 10_000,
        ttl_secs: 300,
        tti_secs: 60,
        redis: None,
    }
}

fn bench_get_no_cache(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (db_repo, _) = rt.block_on(setup_db_repo());

    let post = make_post();
    let id = post.id;
    rt.block_on(db_repo.create(post)).unwrap();

    c.bench_with_input(BenchmarkId::new("get_post", "no_cache"), &id, |b, &id| {
        b.to_async(&rt)
            .iter(|| async { db_repo.get(id.into()).await.unwrap() });
    });

    rt.block_on(db_repo.delete(id.into())).unwrap();
}

fn bench_get_local_cache(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (db_repo, _) = rt.block_on(setup_db_repo());
    let cfg = cache_settings();

    let cached_repo = Arc::new(CachedPostRepository::new(
        db_repo.clone() as Arc<dyn PostRepository>,
        Arc::new(LocalCache::new(&cfg)),
        cfg.ttl(),
    ));

    let post = make_post();
    let id = post.id;
    rt.block_on(cached_repo.create(post)).unwrap();
    rt.block_on(cached_repo.get(id.into())).unwrap();

    c.bench_with_input(
        BenchmarkId::new("get_post", "local_cache"),
        &id,
        |b, &id| {
            b.to_async(&rt)
                .iter(|| async { cached_repo.get(id.into()).await.unwrap() });
        },
    );

    rt.block_on(db_repo.delete(id.into())).unwrap();
}

fn bench_get_redis_cache(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (db_repo, _) = rt.block_on(setup_db_repo());

    let redis_cache = match RedisCache::new("redis://localhost:6380") {
        Ok(r) => Arc::new(r),
        Err(e) => {
            eprintln!("Skipping redis benchmark (redis unavailable): {e}");
            return;
        }
    };

    let cfg = cache_settings();
    let cached_repo = Arc::new(CachedPostRepository::new(
        db_repo.clone() as Arc<dyn PostRepository>,
        redis_cache,
        cfg.ttl(),
    ));

    let post = make_post();
    let id = post.id;
    rt.block_on(cached_repo.create(post)).unwrap();
    rt.block_on(cached_repo.get(id.into())).unwrap();

    c.bench_with_input(
        BenchmarkId::new("get_post", "redis_cache"),
        &id,
        |b, &id| {
            b.to_async(&rt)
                .iter(|| async { cached_repo.get(id.into()).await.unwrap() });
        },
    );

    rt.block_on(db_repo.delete(id.into())).unwrap();
}

fn bench_get_tiered_cache(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (db_repo, _) = rt.block_on(setup_db_repo());
    let cfg = cache_settings();

    let redis_cache = match RedisCache::new("redis://localhost:6380") {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Skipping tiered benchmark (redis unavailable): {e}");
            return;
        }
    };

    let tiered = Arc::new(TieredCache::new(LocalCache::new(&cfg), cfg.ttl()).add_l2(redis_cache));
    let cached_repo = Arc::new(CachedPostRepository::new(
        db_repo.clone() as Arc<dyn PostRepository>,
        tiered,
        cfg.ttl(),
    ));

    let post = make_post();
    let id = post.id;
    rt.block_on(cached_repo.create(post)).unwrap();
    rt.block_on(cached_repo.get(id.into())).unwrap();

    c.bench_with_input(
        BenchmarkId::new("get_post", "tiered_cache"),
        &id,
        |b, &id| {
            b.to_async(&rt)
                .iter(|| async { cached_repo.get(id.into()).await.unwrap() });
        },
    );

    rt.block_on(db_repo.delete(id.into())).unwrap();
}

fn bench_create(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let (db_repo, _) = rt.block_on(setup_db_repo());
    let cfg = cache_settings();

    let cached_repo = Arc::new(CachedPostRepository::new(
        db_repo.clone() as Arc<dyn PostRepository>,
        Arc::new(LocalCache::new(&cfg)),
        cfg.ttl(),
    ));

    let mut group = c.benchmark_group("create_post");

    group.bench_function("no_cache", |b| {
        b.to_async(&rt).iter(|| async {
            let post = make_post();
            let id = post.id;
            db_repo.create(post).await.unwrap();
            db_repo.delete(id.into()).await.unwrap();
        });
    });

    group.bench_function("local_cache", |b| {
        b.to_async(&rt).iter(|| async {
            let post = make_post();
            let id = post.id;
            cached_repo.create(post).await.unwrap();
            db_repo.delete(id.into()).await.unwrap();
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_get_no_cache,
    bench_get_local_cache,
    bench_get_redis_cache,
    bench_get_tiered_cache,
    bench_create,
);
criterion_main!(benches);
