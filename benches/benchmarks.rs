use std::{collections::HashMap, mem::swap, sync::mpsc::channel, thread::available_parallelism};

use serde::{Deserialize, Serialize};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use serde_json::to_string;
use zvariant::{serialized::Context, to_bytes_for_signature, Type, LE};

criterion_group!(
    benches,
    dbus_enc_high_context_switching,
    dbus_enc_low_context_switching,
    json_enc_high_context_switching,
    json_enc_low_context_switching,
    bson_enc_high_context_switching,
    bson_enc_low_context_switching,
);
criterion_main!(benches);

fn dbus_enc_high_context_switching(c: &mut Criterion) {
    const DEFAULT_PARALLELISM: usize = 8;

    let data = Data::new();
    let signature = Data::signature();
    let ctxt = Context::new_dbus(LE, 0);

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
            let (data, _): (Data, _) = encoded
                .deserialize_for_signature(black_box(&signature))
                .unwrap();
            black_box(data);
        })
    });
}

fn dbus_enc_low_context_switching(c: &mut Criterion) {
    let data = Data::new();
    let signature = Data::signature();
    let ctxt = Context::new_dbus(LE, 0);

    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    std::thread::spawn(move || loop {
        let msg = match rx1.recv() {
            Ok(msg) => msg,
            Err(_) => break,
        };
        tx2.send(msg).unwrap();
    });

    c.bench_function("dbus_enc_low_context_switching", |b| {
        b.iter(|| {
            let encoded =
                to_bytes_for_signature(black_box(ctxt), black_box(&signature), black_box(&data))
                    .unwrap();
            tx1.send(encoded).unwrap();

            let encoded = rx2.recv().unwrap();
            let (data, _): (Data, _) = encoded
                .deserialize_for_signature(black_box(&signature))
                .unwrap();
            black_box(data);
        })
    });
}

fn json_enc_high_context_switching(c: &mut Criterion) {
    const DEFAULT_PARALLELISM: usize = 8;

    let data = Data::new();

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
            let data: Data = serde_json::from_slice(encoded.as_bytes()).unwrap();
            black_box(data);
        })
    });
}

fn json_enc_low_context_switching(c: &mut Criterion) {
    let data = Data::new();

    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    std::thread::spawn(move || loop {
        let msg = match rx1.recv() {
            Ok(msg) => msg,
            Err(_) => break,
        };
        tx2.send(msg).unwrap();
    });

    c.bench_function("json_enc_no_context_switching", |b| {
        b.iter(|| {
            let encoded = to_string(black_box(&data)).unwrap();
            tx1.send(encoded).unwrap();

            let encoded = rx2.recv().unwrap();
            let data: Data = serde_json::from_slice(encoded.as_bytes()).unwrap();
            black_box(data);
        })
    });
}

fn bson_enc_high_context_switching(c: &mut Criterion) {
    const DEFAULT_PARALLELISM: usize = 8;

    let data = Data::new();

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
            let data: Data = bson::from_slice(&encoded).unwrap();
            black_box(data);
        })
    });
}

fn bson_enc_low_context_switching(c: &mut Criterion) {
    let data = Data::new();

    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    std::thread::spawn(move || loop {
        let msg = match rx1.recv() {
            Ok(msg) => msg,
            Err(_) => break,
        };
        tx2.send(msg).unwrap();
    });

    c.bench_function("bson_enc_no_context_switching", |b| {
        b.iter(|| {
            let encoded = bson::to_vec(black_box(&data)).unwrap();
            tx1.send(encoded).unwrap();

            let encoded = rx2.recv().unwrap();
            let data: Data = bson::from_slice(&encoded).unwrap();
            black_box(data);
        })
    });
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
struct Data<'f> {
    int1: u64,
    int2: u8,
    bool1: bool,
    string1: &'f str,
    int3: u8,
    string2: &'f str,
    map1: std::collections::HashMap<String, u32>,
    int4: u8,
    string3: &'f str,
    int5: u32,
    map2: std::collections::HashMap<String, u32>,
}
impl Data<'static> {
    pub fn new() -> Self {
        let mut data = Self {
            int1: 42,
            int2: 42,
            bool1: true,
            string1: "Hello, world!",
            int3: 42,
            string2: "Loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong string!",
            map1: HashMap::new(),
            int4: 42,
            string3: "Loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong string!",
            int5: 42,
            map2: HashMap::new(),
        };
        for i in 0..100 {
            let key =
                format!("looooooooooooooooooooooooooooooooooooooooooooooonooooong string key {i}");
            data.map1.insert(key.clone(), i);
            data.map2.insert(key, i);
        }

        data
    }
}
