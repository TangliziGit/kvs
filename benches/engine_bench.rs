use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use kvs::{KvStore, KvsEngine, SledKvsEngine};
use tempfile::TempDir;
use rand::Rng;

pub fn set_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine set method");

    for i in [8, 12].iter() {

        group.bench_with_input(BenchmarkId::new("kvs", i), i, |b, n| b.iter(|| {
            let dir = TempDir::new().unwrap();
            let path = dir.path();
            let mut kvs = KvStore::open(path).unwrap();

            let value = "value".to_string();
            for _i in 0..(1<<n) {
                let key = format!("key{}", i);
                kvs.set(key, value.clone()).unwrap();
            }
        }));

        group.bench_with_input(BenchmarkId::new("sled", i), i, |b, n| b.iter(|| {
            let dir = TempDir::new().unwrap();
            let path = dir.path();
            let mut sled = SledKvsEngine::open(path).unwrap();

            let value = "value".to_string();
            for _i in 0..(1<<n) {
                let key = format!("key{}", i);
                sled.set(key, value.clone()).unwrap();
            }
        }));
    }
}

pub fn set_get_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine get method");

    for i in [8, 12].iter() {

        group.bench_with_input(BenchmarkId::new("kvs", i), i, |b, n| b.iter(|| {
            let dir = TempDir::new().unwrap();
            let path = dir.path();
            let mut kvs = KvStore::open(path).unwrap();

            for _i in 0..(1<<n) {
                let key = format!("key{}", i);
                let value = format!("value{}", i);

                kvs.set(key, value).unwrap();
            }

            let mut rng = rand::thread_rng();
            for _i in 0..(1<<n) {
                let key = format!("key{}", rng.gen_range(0, 1<<n));
                kvs.get(key).unwrap();
            }
        }));

        group.bench_with_input(BenchmarkId::new("sled", i), i, |b, n| b.iter(|| {
            let dir = TempDir::new().unwrap();
            let path = dir.path();
            let mut sled = SledKvsEngine::open(path).unwrap();

            for _i in 0..(1<<n) {
                let key = format!("key{}", i);
                let value = format!("value{}", i);

                sled.set(key, value).unwrap();
            }

            let mut rng = rand::thread_rng();
            for _i in 0..(1<<n) {
                let key = format!("key{}", rng.gen_range(0, 1<<n));
                sled.get(key).unwrap();
            }
        }));
    }
}

pub fn set_get_rm_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("engine rm method");

    for i in [8, 12].iter() {

        group.bench_with_input(BenchmarkId::new("kvs", i), i, |b, n| b.iter(|| {
            let dir = TempDir::new().unwrap();
            let path = dir.path();
            let mut kvs = KvStore::open(path).unwrap();

            for _i in 0..(1<<n) {
                let key = format!("key{}", i);
                let value = format!("value{}", i);

                kvs.set(key, value).unwrap();
            }

            let mut rng = rand::thread_rng();
            for _i in 0..(1<<n) {
                let key = format!("key{}", rng.gen_range(0, 1<<n));
                kvs.get(key).unwrap();
            }

            let mut rest = 1 << (n / 2);
            let mut rng = rand::thread_rng();
            while rest >= 0 {
                let key = format!("key{}", rng.gen_range(0, 1<<n));
                kvs.remove(key);
                rest -= 1;
            }
        }));

        group.bench_with_input(BenchmarkId::new("sled", i), i, |b, n| b.iter(|| {
            let dir = TempDir::new().unwrap();
            let path = dir.path();
            let mut sled = SledKvsEngine::open(path).unwrap();

            for _i in 0..(1<<n) {
                let key = format!("key{}", i);
                let value = format!("value{}", i);

                sled.set(key, value).unwrap();
            }

            let mut rng = rand::thread_rng();
            for _i in 0..(1<<n) {
                let key = format!("key{}", rng.gen_range(0, 1<<n));
                sled.get(key).unwrap();
            }

            let mut rest = 1 << (n / 2);
            let mut rng = rand::thread_rng();
            while rest >= 0 {
                let key = format!("key{}", rng.gen_range(0, 1<<n));
                sled.remove(key);
                rest -= 1;
            }
        }));
    }
}

criterion_group!(benches, set_bench, set_get_bench, set_get_rm_bench);
criterion_main!(benches);