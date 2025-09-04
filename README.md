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

### HashMap-based Data (with long field names)

| Format   | Big (µs) | Small (µs) |
| -------- | -------- | ---------- |
| JSON     | 1464.6   | 304.5      |
| D-Bus    | 1067.4   | 227.3      |
| BSON     | 1841.7   | 384.0      |
| CBOR     | 1061.3   | 211.3      |
| Bincode  |  635.9   | 127.2      |
| Bitcode  |  573.5   | 116.1      |

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

- JSON performs competitively, especially for small payloads.
- Binary formats like Bincode and Bitcode show ~2x performance advantage.

#### Vector-based Data

- Bincode and Bitcode are dramatically faster (8-15x faster for big payloads).
- For small vector payloads, binary formats are 15x faster than JSON.
- D-Bus outperforms JSON for vector data.

### Size

The size comparison heavily depends on the data structure:

#### HashMap-based Data (with long field names)

- Binary formats achieve only modest size reduction (2-3% savings).
- Bitcode is most compact at ~97% of JSON size.
- BSON and D-Bus actually produce larger outputs than JSON.
- JSON is surprisingly efficient for data with many string keys and long field names.

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
