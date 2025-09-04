# Benchmarking JSON vs. binary formats

This repo benchmarks performance of JSON vs. binary format encoding using serde, focusing on the
speed of encoding and decoding for different payload sizes.

## Running the benchmarks

```bash
cargo +nightly bench
```

## Running the size analysis

```bash
cargo run
```

This will output a comparison of encoded sizes for each format.

## Results

The results on my machines (from one of the runs) are as follows for different formats:

### HashMap-based Data

| Format    | Big (µs) | Small (µs) |
| --------- | -------- | ---------- |
| JSON      |  492.8   |  88.6      |
| D-Bus     |  666.8   | 135.6      |
| BSON      |  714.3   | 139.3      |
| CBOR      |  705.1   | 135.6      |
| Bincode   |  305.0   |  57.3      |
| Bitcode   |  265.7   |  54.4      |

### Vector-based Data (arrays of structs)

| Format    | Big (µs) | Small (µs) |
| --------- | -------- | ---------- |
| JSON      | 6039.2   | 328.2      |
| D-Bus     | 4242.5   | 179.4      |
| BSON      | 7943.4   | 420.5      |
| CBOR      | 9814.0   | 516.2      |
| Bincode   |  759.7   |  21.6      |
| Bitcode   |  570.5   |  21.8      |

Not only YMMV, but also the results are not very consistent across runs. They depends a lot on the
system load (which can fluctuate a lot). So it's best to run the benchmarks multiple times and take
the average.

## Observations

### Performance

#### HashMap-based Data

- Bitcode is the fastest, ~1.8x faster than JSON for big payloads.
- Bincode shows ~1.6x performance advantage over JSON.
- D-Bus, BSON, and CBOR are actually slower than JSON for HashMap data.

#### Vector-based Data

- Bincode and Bitcode are dramatically faster (8-15x faster for big payloads).
- For small vector payloads, binary formats are 15x faster than JSON.
- D-Bus outperforms JSON for vector data.

### Size

The size comparison heavily depends on the data structure:

#### HashMap-based Data

- Binary formats now achieve significant size reduction (30-37% savings).
- Bitcode is most compact at ~66% of JSON size.
- BSON and D-Bus produce larger outputs than JSON (~121% and ~120% respectively).
- CBOR achieves ~78% of JSON size.
- Bincode achieves ~70% of JSON size.

#### Vector-based Data (arrays of structs)

- Binary formats achieve dramatic size reductions (up to 87% savings!).
- Bitcode compresses to just 18.5% of JSON size for big payloads.
- Bincode achieves 24.2% of JSON size.
- Even D-Bus achieves good compression at 40.4% of JSON size.
- Binary formats excel when field names don't need to be repeated.

### Key Takeaway

The choice between JSON and binary formats depends heavily on your data structure:
- Use JSON when dealing with hashmaps, dynamic keys, or when human readability matters.
- Use binary formats when dealing with arrays of structs, time-series data, or when size/performance is critical.
- Data structure design has more impact on serialization efficiency than the format itself.
