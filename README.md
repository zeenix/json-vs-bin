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

### JSON

| Size  | Time (µs) |
| ----- | --------- |
| Big   | 1464.6    |
| Small |  304.5    |

### D-Bus

| Size  | Time (µs) |
| ----- | --------- |
| Big   | 1067.4    |
| Small |  227.3    |

### BSON

| Size  | Time (µs) |
| ----- | --------- |
| Big   | 1841.7    |
| Small |  384.0    |

### CBOR

| Size  | Time (µs) |
| ----- | --------- |
| Big   | 1061.3    |
| Small |  211.3    |

### Bincode

| Size  | Time (µs) |
| ----- | --------- |
| Big   |  635.9    |
| Small |  127.2    |

### Bitcode

| Size  | Time (µs) |
| ----- | --------- |
| Big   |  573.5    |
| Small |  116.1    |

Not only YMMV, but also the results are not very consistent across runs. They depends a lot on the
system load (which can fluctuate a lot). So it's best to run the benchmarks multiple times and take
the average.

## Observations

### Performance
- For small payloads, JSON performs competitively with binary formats.
- JSON encoding is most efficient when there are a lot of hashmaps involved, especially with string keys.
- Binary formats like Bincode and Bitcode show significant performance advantages for both payload sizes.

### Size
- Binary formats achieve only modest size reduction compared to JSON (around 2-3% savings).
- Bitcode produces the most compact encoding, saving about 3% compared to JSON.
- BSON and D-Bus actually produce larger outputs than JSON due to their metadata overhead.
- The small compression gains suggest that JSON's text format is reasonably efficient for data with many string keys.
