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
| JSON      |  340.2   |  59.2      |
| D-Bus     |  399.1   |  78.6      |
| BSON      |  432.8   |  83.0      |
| Bincode   |  156.1   |  27.4      |
| Bitcode   |  116.5   |  22.6      |
| Postcard  |  179.4   |  34.4      |

### Vector-based Data (arrays of structs)

| Format    | Big (µs) | Small (µs) |
| --------- | -------- | ---------- |
| JSON      | 5690.3   | 317.9      |
| D-Bus     | 3649.8   | 180.5      |
| BSON      | 7443.1   | 418.9      |
| Bincode   |  459.4   |  20.5      |
| Bitcode   |  360.2   |  22.1      |
| Postcard  |  577.9   |  24.6      |

Not only YMMV, but also the results are not very consistent across runs. They depends a lot on the
system load (which can fluctuate a lot). So it's best to run the benchmarks multiple times and take
the average.

## Observations

### Performance

#### HashMap-based Data

- Bitcode is the fastest, ~2.9x faster than JSON for big payloads.
- Bincode shows ~2.2x performance advantage over JSON.
- Postcard shows ~1.9x performance advantage over JSON.
- D-Bus and BSON are slower than JSON for HashMap data.

#### Vector-based Data

- Bitcode, Bincode, and Postcard are dramatically faster (10-16x faster for big payloads).
- For small vector payloads, these binary formats are 14-15x faster than JSON.
- D-Bus outperforms JSON by ~1.6x for vector data.

### Size

The size comparison heavily depends on the data structure:

#### HashMap-based Data

- Binary formats now achieve significant size reduction (30-37% savings).
- Bitcode is most compact at ~66% of JSON size.
- Bincode achieves ~70% of JSON size.
- Postcard achieves ~71% of JSON size.
- BSON and D-Bus produce larger outputs than JSON (~121% and ~120% respectively).

#### Vector-based Data (arrays of structs)

- Binary formats achieve dramatic size reductions (up to 87% savings!).
- Bitcode compresses to just 18.5% of JSON size for big payloads.
- Postcard achieves 22.8% of JSON size.
- Bincode achieves 24.2% of JSON size.
- Even D-Bus achieves good compression at 40.4% of JSON size.
- Binary formats excel when field names don't need to be repeated.

### Key Takeaway

The choice between JSON and binary formats depends heavily on your data structure:
- Use JSON when dealing with hashmaps, dynamic keys, or when human readability matters.
- Use binary formats when dealing with arrays of structs, time-series data, or when size/performance is critical.
- Data structure design has more impact on serialization efficiency than the format itself.
