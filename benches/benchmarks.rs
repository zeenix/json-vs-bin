/// This benchmark is to compare the performance of JSON and a few binary formats.
use std::iter;

use serde::{de::DeserializeOwned, Serialize};

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
        cbor,
        bincode,
        bitcode,
        postcard,
        dbus_vector,
        json_vector,
        simd_json_vector,
        bson_vector,
        cbor_vector,
        bincode_vector,
        bitcode_vector,
        postcard_vector
}
criterion_main!(benches);

fn dbus(c: &mut Criterion) {
    let dbus = formats::DBus::new();
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigData>| dbus.encode_big(black_box(data));
    let dec_fn = |bytes: &[u8]| dbus.decode_big(bytes);
    bench_it(c, data, enc_fn, dec_fn, "dbus_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallData>| dbus.encode_small(black_box(data));
    let dec_fn = |bytes: &[u8]| dbus.decode_small(bytes);
    bench_it(c, data, enc_fn, dec_fn, "dbus_small");
}

fn json(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigData>| formats::Json::encode_big(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Json::decode_big(bytes);
    bench_it(c, data, enc_fn, dec_fn, "json_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallData>| formats::Json::encode_small(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Json::decode_small(bytes);
    bench_it(c, data, enc_fn, dec_fn, "json_small");
}

fn simd_json(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigData>| formats::SimdJson::encode_big(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::SimdJson::decode_big(bytes);
    bench_it(c, data, enc_fn, dec_fn, "simd_json_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallData>| formats::SimdJson::encode_small(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::SimdJson::decode_small(bytes);
    bench_it(c, data, enc_fn, dec_fn, "simd_json_small");
}

fn bson(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigData>| formats::Bson::encode_big(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Bson::decode_big(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bson_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallData>| formats::Bson::encode_small(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Bson::decode_small(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bson_small");
}

fn cbor(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigData>| formats::Cbor::encode_big(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Cbor::decode_big(bytes);
    bench_it(c, data, enc_fn, dec_fn, "cbor_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallData>| formats::Cbor::encode_small(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Cbor::decode_small(bytes);
    bench_it(c, data, enc_fn, dec_fn, "cbor_small");
}

fn bincode(c: &mut Criterion) {
    let bincode = formats::Bincode::new();
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigData>| bincode.encode_big(black_box(data));
    let dec_fn = |bytes: &[u8]| bincode.decode_big(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bincode_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallData>| bincode.encode_small(black_box(data));
    let dec_fn = |bytes: &[u8]| bincode.decode_small(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bincode_small");
}

fn bitcode(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigData>| formats::Bitcode::encode_big(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Bitcode::decode_big(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bitcode_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallData>| formats::Bitcode::encode_small(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Bitcode::decode_small(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bitcode_small");
}

fn postcard(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigData>| formats::Postcard::encode_big(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Postcard::decode_big(bytes);
    bench_it(c, data, enc_fn, dec_fn, "postcard_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallData>| formats::Postcard::encode_small(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Postcard::decode_small(bytes);
    bench_it(c, data, enc_fn, dec_fn, "postcard_small");
}

fn dbus_vector(c: &mut Criterion) {
    let dbus = formats::DBus::new();
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigVectorData>| dbus.encode_big_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| dbus.decode_big_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "dbus_big_vector");

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallVectorData>| dbus.encode_small_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| dbus.decode_small_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "dbus_small_vector");
}

fn json_vector(c: &mut Criterion) {
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigVectorData>| formats::Json::encode_big_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Json::decode_big_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "json_big_vector");

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallVectorData>| formats::Json::encode_small_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Json::decode_small_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "json_small_vector");
}

fn simd_json_vector(c: &mut Criterion) {
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigVectorData>| formats::SimdJson::encode_big_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::SimdJson::decode_big_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "simd_json_big_vector");

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn =
        |data: &Vec<SmallVectorData>| formats::SimdJson::encode_small_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::SimdJson::decode_small_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "simd_json_small_vector");
}

fn bson_vector(c: &mut Criterion) {
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigVectorData>| formats::Bson::encode_big_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Bson::decode_big_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bson_big_vector");

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallVectorData>| formats::Bson::encode_small_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Bson::decode_small_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bson_small_vector");
}

fn cbor_vector(c: &mut Criterion) {
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigVectorData>| formats::Cbor::encode_big_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Cbor::decode_big_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "cbor_big_vector");

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallVectorData>| formats::Cbor::encode_small_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Cbor::decode_small_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "cbor_small_vector");
}

fn bincode_vector(c: &mut Criterion) {
    let bincode = formats::Bincode::new();
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigVectorData>| bincode.encode_big_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| bincode.decode_big_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bincode_big_vector");

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallVectorData>| bincode.encode_small_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| bincode.decode_small_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bincode_small_vector");
}

fn bitcode_vector(c: &mut Criterion) {
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigVectorData>| formats::Bitcode::encode_big_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Bitcode::decode_big_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bitcode_big_vector");

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn =
        |data: &Vec<SmallVectorData>| formats::Bitcode::encode_small_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Bitcode::decode_small_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "bitcode_small_vector");
}

fn postcard_vector(c: &mut Criterion) {
    let data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigVectorData>| formats::Postcard::encode_big_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Postcard::decode_big_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "postcard_big_vector");

    let data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn =
        |data: &Vec<SmallVectorData>| formats::Postcard::encode_small_vector(black_box(data));
    let dec_fn = |bytes: &[u8]| formats::Postcard::decode_small_vector(bytes);
    bench_it(c, data, enc_fn, dec_fn, "postcard_small_vector");
}

fn bench_it<D, EncFn, DecFn>(
    c: &mut Criterion,
    data: D,
    enc_fn: EncFn,
    dec_fn: DecFn,
    bench_name: &str,
) where
    D: Serialize + DeserializeOwned,
    EncFn: Fn(&D) -> Vec<u8>,
    DecFn: Fn(&[u8]) -> D,
{
    let mut group = c.benchmark_group("encoding_decoding");
    group.measurement_time(std::time::Duration::from_secs(20));
    group.bench_function(bench_name, |b| {
        b.iter(|| {
            let encoded = enc_fn(black_box(&data));
            let data: D = dec_fn(&encoded);
            black_box(data);
        })
    });
}
