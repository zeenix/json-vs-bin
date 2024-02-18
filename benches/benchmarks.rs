use std::{collections::HashMap, mem::swap, sync::mpsc::channel};

use serde::{Deserialize, Serialize};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use zvariant::{serialized::Context, to_bytes_for_signature, Type, LE};

fn dbus_enc_context_switching(c: &mut Criterion) {
    #[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
    struct Foo<'f> {
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
    let mut foo = Foo {
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
        foo.map1.insert(key.clone(), i);
        foo.map2.insert(key, i);
    }
    let signature = Foo::signature();
    let ctxt = Context::new_dbus(LE, 0);

    // Create 8 threads and channels, with main thread receiving back what it sends to the first
    // channel, from the last channel in the chain.
    let (first_tx, mut last_rx) = channel();
    for _ in 0..NUM_THREADS {
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

    c.bench_function("dbus_enc_context_switching", |b| {
        b.iter(|| {
            let encoded =
                to_bytes_for_signature(black_box(ctxt), black_box(&signature), black_box(&foo))
                    .unwrap();
            first_tx.send(encoded).unwrap();

            let encoded = last_rx.recv().unwrap();
            let (s, _): (Foo, _) = encoded
                .deserialize_for_signature(black_box(&signature))
                .unwrap();
            black_box(s);
        })
    });
}

criterion_group!(benches, dbus_enc_context_switching);
criterion_main!(benches);

const NUM_THREADS: usize = 8;
