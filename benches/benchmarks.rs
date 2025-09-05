/// This benchmark is to compare the performance of JSON and a few binary formats.
use std::iter;

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

use json_vs_bin::{
    data::{BigData, SmallData},
    formats,
    vector_data::{BigVectorData, SmallVectorData},
};

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(std::time::Duration::from_secs(30))
        .sample_size(60);
    targets = dbus,
        json,
        simd_json,
        bson,
        bincode,
        bitcode,
        postcard,
        dbus_vector,
        json_vector,
        simd_json_vector,
        bson_vector,
        bincode_vector,
        bitcode_vector,
        postcard_vector
}
criterion_main!(benches);

fn dbus(c: &mut Criterion) {
    let dbus = formats::DBus::new();
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();

    let mut group = c.benchmark_group("dbus");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = dbus.encode_big(black_box(&data));
            let decoded = dbus.decode_big(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("dbus");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = dbus.encode_small(black_box(&data));
            let decoded = dbus.decode_small(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}

fn json(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();

    let mut group = c.benchmark_group("json");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = formats::Json::encode_big(black_box(&data));
            let decoded = formats::Json::decode_big(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("json");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = formats::Json::encode_small(black_box(&data));
            let decoded = formats::Json::decode_small(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}

fn simd_json(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();

    let mut group = c.benchmark_group("simd_json");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = formats::SimdJson::encode_big(black_box(&data));
            let mut buf = encoded.clone();
            let decoded = formats::SimdJson::decode_big(black_box(&mut buf));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("simd_json");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = formats::SimdJson::encode_small(black_box(&data));
            let mut buf = encoded.clone();
            let decoded = formats::SimdJson::decode_small(black_box(&mut buf));
            black_box(decoded);
        })
    });
    group.finish();
}

fn bson(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();

    let mut group = c.benchmark_group("bson");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = formats::Bson::encode_big(black_box(&data));
            let decoded = formats::Bson::decode_big(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("bson");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = formats::Bson::encode_small(black_box(&data));
            let decoded = formats::Bson::decode_small(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}

// CBOR removed - ciborium has serde trait limitations with &str fields

fn bincode(c: &mut Criterion) {
    let bincode = formats::Bincode::new();
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();

    let mut group = c.benchmark_group("bincode");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = bincode.encode_big(black_box(&data));
            let decoded = bincode.decode_big(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("bincode");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = bincode.encode_small(black_box(&data));
            let decoded = bincode.decode_small(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}

fn bitcode(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();

    let mut group = c.benchmark_group("bitcode");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = formats::Bitcode::encode_big(black_box(&data));
            let decoded = formats::Bitcode::decode_big(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("bitcode");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = formats::Bitcode::encode_small(black_box(&data));
            let decoded = formats::Bitcode::decode_small(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}

fn postcard(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();

    let mut group = c.benchmark_group("postcard");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = formats::Postcard::encode_big(black_box(&data));
            let decoded = formats::Postcard::decode_big(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("postcard");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = formats::Postcard::encode_small(black_box(&data));
            let decoded = formats::Postcard::decode_small(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}

fn dbus_vector(c: &mut Criterion) {
    let dbus = formats::DBus::new();
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("dbus_vector");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = dbus.encode_big_vector(black_box(&data));
            let decoded = dbus.decode_big_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("dbus_vector");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = dbus.encode_small_vector(black_box(&data));
            let decoded = dbus.decode_small_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}

fn json_vector(c: &mut Criterion) {
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("json_vector");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = formats::Json::encode_big_vector(black_box(&data));
            let decoded = formats::Json::decode_big_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("json_vector");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = formats::Json::encode_small_vector(black_box(&data));
            let decoded = formats::Json::decode_small_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}

fn simd_json_vector(c: &mut Criterion) {
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("simd_json_vector");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = formats::SimdJson::encode_big_vector(black_box(&data));
            let mut buf = encoded.clone();
            let decoded = formats::SimdJson::decode_big_vector(black_box(&mut buf));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("simd_json_vector");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = formats::SimdJson::encode_small_vector(black_box(&data));
            let mut buf = encoded.clone();
            let decoded = formats::SimdJson::decode_small_vector(black_box(&mut buf));
            black_box(decoded);
        })
    });
    group.finish();
}

fn bson_vector(c: &mut Criterion) {
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("bson_vector");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = formats::Bson::encode_big_vector(black_box(&data));
            let decoded = formats::Bson::decode_big_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("bson_vector");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = formats::Bson::encode_small_vector(black_box(&data));
            let decoded = formats::Bson::decode_small_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}

// CBOR vector removed - ciborium has serde trait limitations with &str fields

fn bincode_vector(c: &mut Criterion) {
    let bincode = formats::Bincode::new();
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("bincode_vector");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = bincode.encode_big_vector(black_box(&data));
            let decoded = bincode.decode_big_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("bincode_vector");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = bincode.encode_small_vector(black_box(&data));
            let decoded = bincode.decode_small_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}

fn bitcode_vector(c: &mut Criterion) {
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("bitcode_vector");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = formats::Bitcode::encode_big_vector(black_box(&data));
            let decoded = formats::Bitcode::decode_big_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("bitcode_vector");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = formats::Bitcode::encode_small_vector(black_box(&data));
            let decoded = formats::Bitcode::decode_small_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}

fn postcard_vector(c: &mut Criterion) {
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("postcard_vector");
    group.bench_function("big", |b| {
        b.iter(|| {
            let encoded = formats::Postcard::encode_big_vector(black_box(&data));
            let decoded = formats::Postcard::decode_big_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("postcard_vector");
    group.bench_function("small", |b| {
        b.iter(|| {
            let encoded = formats::Postcard::encode_small_vector(black_box(&data));
            let decoded = formats::Postcard::decode_small_vector(black_box(&encoded));
            black_box(decoded);
        })
    });
    group.finish();
}
