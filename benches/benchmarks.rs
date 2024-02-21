/// This benchmark is to compare the performance of JSON and a few binary formats.
///
/// TODO:
/// * Also, benchmark with tokio tasks instead of threads.
use std::{
    collections::HashMap, iter, mem::swap, sync::mpsc::channel, thread::available_parallelism,
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use serde_json::to_string;
use zvariant::{serialized::Context, to_bytes_for_signature, Type, LE};

criterion_group!(benches, dbus, json, bson, cbor);
criterion_main!(benches);

fn dbus(c: &mut Criterion) {
    let signature = <Vec<BigData>>::signature();
    let ctxt = Context::new_dbus(LE, 0);
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigData>| {
        to_bytes_for_signature(black_box(ctxt), black_box(&signature), black_box(data))
            .unwrap()
            .to_vec()
    };
    let dec_fn = |data: &[u8]| {
        let encoded = zvariant::serialized::Data::new(data, ctxt);
        let (data, _): (Vec<BigData>, _) = encoded
            .deserialize_for_signature(black_box(&signature))
            .unwrap();
        data
    };
    bench_it(c, data, enc_fn, dec_fn, "dbus_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallData>| {
        to_bytes_for_signature(black_box(ctxt), black_box(&signature), black_box(data))
            .unwrap()
            .to_vec()
    };
    let dec_fn = |data: &[u8]| {
        let encoded = zvariant::serialized::Data::new(data, ctxt);
        let (data, _): (Vec<SmallData>, _) = encoded
            .deserialize_for_signature(black_box(&signature))
            .unwrap();
        data
    };
    bench_it(c, data, enc_fn, dec_fn, "dbus_small");
}

fn json(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigData>| to_string(black_box(&data)).unwrap().into_bytes();
    let dec_fn = |encoded: &[u8]| serde_json::from_slice(encoded).unwrap();
    bench_it(c, data, enc_fn, dec_fn, "json_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallData>| to_string(black_box(&data)).unwrap().into_bytes();
    let dec_fn = |encoded: &[u8]| serde_json::from_slice(encoded).unwrap();
    bench_it(c, data, enc_fn, dec_fn, "json_small");
}

fn bson(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    // BSON can't handle arrays at the top level, so we wrap it in a struct.
    #[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
    struct Foo<D> {
        data: Vec<D>,
    }
    let data = Foo { data };

    let enc_fn = |data: &Foo<BigData>| bson::to_vec(black_box(&data)).unwrap();
    let dec_fn = |encoded: &[u8]| bson::from_slice(encoded).unwrap();
    bench_it(c, data, enc_fn, dec_fn, "bson_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let data = Foo { data };
    let enc_fn = |data: &Foo<SmallData>| bson::to_vec(black_box(&data)).unwrap();
    let dec_fn = |encoded: &[u8]| bson::from_slice(encoded).unwrap();
    bench_it(c, data, enc_fn, dec_fn, "bson_small");
}

fn cbor(c: &mut Criterion) {
    let data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let enc_fn = |data: &Vec<BigData>| {
        let mut encoded = Vec::new();
        ciborium::into_writer(black_box(&data), black_box(&mut encoded)).unwrap();

        encoded
    };
    let dec_fn = |encoded: &[u8]| ciborium::from_reader(black_box(&encoded[..])).unwrap();
    bench_it(c, data, enc_fn, dec_fn, "cbor_big");

    let data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();
    let enc_fn = |data: &Vec<SmallData>| {
        let mut encoded = Vec::new();
        ciborium::into_writer(black_box(&data), black_box(&mut encoded)).unwrap();

        encoded
    };
    let dec_fn = |encoded: &[u8]| ciborium::from_reader(black_box(&encoded[..])).unwrap();
    bench_it(c, data, enc_fn, dec_fn, "cbor_small");
}

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
    let mut group = c.benchmark_group("no_context_switching");
    group.measurement_time(std::time::Duration::from_secs(20));
    group.bench_function(bench_name, |b| {
        b.iter(|| {
            let encoded = enc_fn(black_box(&data));
            let data: D = dec_fn(&encoded);
            black_box(data);
        })
    });
    drop(group);

    let (first_tx, last_rx) = setup_channels_and_threads();

    let mut group = c.benchmark_group("high_context_switching_threads");
    group.measurement_time(std::time::Duration::from_secs(20));
    group.bench_function(bench_name, |b| {
        b.iter(|| {
            let encoded = enc_fn(black_box(&data));
            first_tx.send(encoded.to_vec()).unwrap();

            let encoded = last_rx.recv().unwrap();
            let data: D = dec_fn(&encoded);
            black_box(data);
        })
    });
    drop(group);

    let rt = tokio::runtime::Runtime::new().unwrap();
    let (first_tx, mut last_rx) = setup_channels_and_tokio(&rt);
    let mut group = c.benchmark_group("high_context_switching_tasks");
    group.measurement_time(std::time::Duration::from_secs(20));
    group.bench_function(bench_name, |b| {
        b.iter(|| {
            rt.block_on(async {
                let encoded = enc_fn(black_box(&data));
                first_tx.send(encoded.to_vec()).await.unwrap();

                let encoded = last_rx.recv().await.unwrap();
                let data: D = dec_fn(&encoded);
                black_box(data);
            })
        })
    });
}

fn setup_channels_and_threads() -> (
    std::sync::mpsc::Sender<Vec<u8>>,
    std::sync::mpsc::Receiver<Vec<u8>>,
) {
    // Create 8 threads and channels, with main thread receiving back what it sends to the first
    // channel, from the last channel in the chain.
    let (first_tx, mut last_rx) = channel();
    for _ in 0..available_parallelism()
        .map(Into::into)
        .unwrap_or(DEFAULT_PARALLELISM)
    {
        let (tx, mut rx) = channel();
        swap(&mut last_rx, &mut rx);
        std::thread::spawn(move || loop {
            let msg = match rx.recv() {
                Ok(msg) => msg,
                Err(_) => break,
            };
            tx.send(msg).unwrap();
        });
    }

    (first_tx, last_rx)
}

fn setup_channels_and_tokio(
    rt: &tokio::runtime::Runtime,
) -> (
    tokio::sync::mpsc::Sender<Vec<u8>>,
    tokio::sync::mpsc::Receiver<Vec<u8>>,
) {
    let (first_tx, mut last_rx) = tokio::sync::mpsc::channel(1);
    for _ in 0..available_parallelism()
        .map(Into::into)
        .unwrap_or(DEFAULT_PARALLELISM)
    {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        swap(&mut last_rx, &mut rx);
        rt.spawn(async move {
            loop {
                let msg = match rx.recv().await {
                    Some(msg) => msg,
                    None => break,
                };
                tx.send(msg).await.unwrap();
            }
        });
    }

    (first_tx, last_rx)
}

const DEFAULT_PARALLELISM: usize = 8;
