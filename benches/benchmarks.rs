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
        dbus_vector,
        json_vector,
        simd_json_vector,
        bson_vector,
        cbor_vector,
        bincode_vector,
        bitcode_vector
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

// Data structures removed - now using shared library from json_vs_bin crate
/*
#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
struct BigData {
    int1_loooooooooooooooooooooooooooooooooooong_name: u64,
    int2_loooooooooooooooooooooooooooooooooooooong_name: u8,
    bool1_looooooooooooooooooooooong_name: bool,
    string1: String,
    int3_loooooooooooooooooooooooooooooooooooong_naaaaame: u8,
    string2: String,
    map1_loooooooooooooooooooooooong_name: std::collections::HashMap<String, u32>,
    int4: u8,
    string3: String,
    int5: u32,
    map2_also_loooooooooooooong_name: std::collections::HashMap<String, u32>,
    // Repeat previous fields 4 times more but with different names
    int6_loooooooooooooooooooooooooooooooooooong_name: u64,
    int7_loooooooooooooooooooooooooooooooooooooong_name: u8,
    bool2_looooooooooooooooooooooong_name: bool,
    string4: String,
    int8_loooooooooooooooooooooooooooooooooooong_naaaaame: u8,
    string5: String,
    map3_loooooooooooooooooooooooong_name: std::collections::HashMap<String, u32>,
    int9: u8,
    string6: String,
    int10: u32,
    map4_also_loooooooooooooong_name: std::collections::HashMap<String, u32>,
    int11_loooooooooooooooooooooooooooooooooooong_name: u64,
    int12_loooooooooooooooooooooooooooooooooooooong_name: u8,
    bool3_looooooooooooooooooooooong_name: bool,
    string7: String,
    int13_loooooooooooooooooooooooooooooooooooong_naaaaame: u8,
    string8: String,
    map5_loooooooooooooooooooooooong_name: std::collections::HashMap<String, u32>,
}
impl BigData {
    pub fn new() -> Self {
        let string = iter::repeat('o').take(250).collect::<String>();
        let mut map = HashMap::new();
        for i in 0..100 {
            let mut key = string.clone();
            key.push(unsafe { char::from_u32_unchecked(i as u32 + 64) });
            map.insert(key.clone(), i);
        }
        Self {
            int1_loooooooooooooooooooooooooooooooooooong_name: 42,
            int2_loooooooooooooooooooooooooooooooooooooong_name: 42,
            bool1_looooooooooooooooooooooong_name: true,
            string1: "Hello, world!".to_string(),
            int3_loooooooooooooooooooooooooooooooooooong_naaaaame: 42,
            string2: string.clone(),
            map1_loooooooooooooooooooooooong_name: map.clone(),
            int4: 42,
            string3: string.clone(),
            int5: 42,
            map2_also_loooooooooooooong_name: map.clone(),
            int6_loooooooooooooooooooooooooooooooooooong_name: 42,
            int7_loooooooooooooooooooooooooooooooooooooong_name: 42,
            bool2_looooooooooooooooooooooong_name: true,
            string4: "Hello, world!".to_string(),
            int8_loooooooooooooooooooooooooooooooooooong_naaaaame: 42,
            string5: string.clone(),
            map3_loooooooooooooooooooooooong_name: map.clone(),
            int9: 42,
            string6: string.clone(),
            int10: 42,
            map4_also_loooooooooooooong_name: map.clone(),
            int11_loooooooooooooooooooooooooooooooooooong_name: 42,
            int12_loooooooooooooooooooooooooooooooooooooong_name: 42,
            bool3_looooooooooooooooooooooong_name: true,
            string7: "Hello, world!".to_string(),
            int13_loooooooooooooooooooooooooooooooooooong_naaaaame: 42,
            string8: string.clone(),
            map5_loooooooooooooooooooooooong_name: map,
        }
    }
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
struct SmallData {
    int1_loooooooooooooooooooooooooooooooooooong_name: u64,
    int2_loooooooooooooooooooooooooooooooooooooong_name: u8,
    bool1_looooooooooooooooooooooong_name: bool,
    string1: String,
    int3_loooooooooooooooooooooooooooooooooooong_naaaaame: u8,
    string2: String,
    map1_loooooooooooooooooooooooong_name: std::collections::HashMap<String, u32>,
}

impl SmallData {
    pub fn new() -> Self {
        let string = iter::repeat('o').take(250).collect::<String>();
        let mut map = HashMap::new();
        for i in 0..100 {
            let mut key = string.clone();
            key.push(unsafe { char::from_u32_unchecked(i as u32 + 64) });
            map.insert(key.clone(), i);
        }
        Self {
            int1_loooooooooooooooooooooooooooooooooooong_name: 42,
            int2_loooooooooooooooooooooooooooooooooooooong_name: 42,
            bool1_looooooooooooooooooooooong_name: true,
            string1: "Hello, world!".to_string(),
            int3_loooooooooooooooooooooooooooooooooooong_naaaaame: 42,
            string2: string.clone(),
            map1_loooooooooooooooooooooooong_name: map,
        }
    }
}
*/

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
