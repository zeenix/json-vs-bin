/// This benchmark is to compare the performance of JSON and a few binary formats.
///
/// TODO:
/// * Also, benchmark with tokio tasks instead of threads.
use std::{
    collections::HashMap, iter, mem::swap, sync::mpsc::channel, thread::available_parallelism,
};

use serde::{Deserialize, Serialize};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use serde_json::to_string;
use zvariant::{serialized::Context, to_bytes_for_signature, Type, LE};

criterion_group!(benches, dbus_enc, json_enc, bson_enc, cbor_enc);
criterion_main!(benches);

fn dbus_enc(c: &mut Criterion) {
    let data = iter::repeat_with(Data::new).take(10).collect::<Vec<_>>();
    let signature = <Vec<Data>>::signature();
    let ctxt = Context::new_dbus(LE, 0);

    c.bench_function("dbus_enc_no_context_switching", |b| {
        b.iter(|| {
            let encoded =
                to_bytes_for_signature(black_box(ctxt), black_box(&signature), black_box(&data))
                    .unwrap();

            let (data, _): (Vec<Data>, _) = encoded
                .deserialize_for_signature(black_box(&signature))
                .unwrap();
            black_box(data);
        })
    });

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

    c.bench_function("dbus_enc_high_context_switching", |b| {
        b.iter(|| {
            let encoded =
                to_bytes_for_signature(black_box(ctxt), black_box(&signature), black_box(&data))
                    .unwrap();
            first_tx.send(encoded).unwrap();

            let encoded = last_rx.recv().unwrap();
            let (data, _): (Vec<Data>, _) = encoded
                .deserialize_for_signature(black_box(&signature))
                .unwrap();
            black_box(data);
        })
    });
}

fn json_enc(c: &mut Criterion) {
    let data = iter::repeat_with(Data::new).take(10).collect::<Vec<_>>();

    c.bench_function("json_enc_no_context_switching", |b| {
        b.iter(|| {
            let encoded = to_string(black_box(&data)).unwrap();
            let data: Vec<Data> = serde_json::from_slice(encoded.as_bytes()).unwrap();
            black_box(data);
        })
    });

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

    c.bench_function("json_enc_high_context_switching", |b| {
        b.iter(|| {
            let encoded = to_string(black_box(&data)).unwrap();
            first_tx.send(encoded).unwrap();

            let encoded = last_rx.recv().unwrap();
            let data: Vec<Data> = serde_json::from_slice(encoded.as_bytes()).unwrap();
            black_box(data);
        })
    });
}

fn bson_enc(c: &mut Criterion) {
    let data = iter::repeat_with(Data::new).take(10).collect::<Vec<_>>();
    // BSON can't handle arrays at the top level, so we wrap it in a struct.
    #[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
    struct Foo {
        data: Vec<Data>,
    }
    let data = Foo { data };

    c.bench_function("bson_enc_no_context_switching", |b| {
        b.iter(|| {
            let encoded = bson::to_vec(black_box(&data)).unwrap();
            let data: Foo = bson::from_slice(&encoded).unwrap();
            black_box(data);
        })
    });

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

    c.bench_function("bson_enc_high_context_switching", |b| {
        b.iter(|| {
            let encoded = bson::to_vec(black_box(&data)).unwrap();
            first_tx.send(encoded).unwrap();

            let encoded = last_rx.recv().unwrap();
            let data: Foo = bson::from_slice(&encoded).unwrap();
            black_box(data);
        })
    });
}

fn cbor_enc(c: &mut Criterion) {
    let data = iter::repeat_with(Data::new).take(10).collect::<Vec<_>>();

    c.bench_function("cbor_enc_no_context_switching", |b| {
        b.iter(|| {
            let mut encoded = Vec::new();
            ciborium::into_writer(black_box(&data), black_box(&mut encoded)).unwrap();
            let data: Vec<Data> = ciborium::from_reader(black_box(&encoded[..])).unwrap();
            black_box(data);
        })
    });

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

    c.bench_function("cbor_enc_high_context_switching", |b| {
        b.iter(|| {
            let mut encoded = Vec::new();
            ciborium::into_writer(black_box(&data), black_box(&mut encoded)).unwrap();
            first_tx.send(encoded).unwrap();

            let encoded = last_rx.recv().unwrap();
            let data: Vec<Data> = ciborium::from_reader(black_box(&encoded[..])).unwrap();
            black_box(data);
        })
    });
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
struct Data {
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
impl Data {
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

const DEFAULT_PARALLELISM: usize = 8;
