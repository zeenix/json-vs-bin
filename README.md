# Benchmarking JSON vs. binary formats

This repo benchmarks performance of JSON vs. binary format encoding using serde, focusing on the
speed of encoding and decoding for different payload sizes.

## Running the benchmarks

```bash
cargo +nightly bench
```

## Results

The results on my machines (from one of the runs) are as follows for different formats:

### JSON

| Size  | Time (µs) |
| ----- | --------- |
| Big   | 2013.8    |
| Small |  441.4    |

### D-Bus

| Size  | Time (µs) |
| ----- | --------- |
| Big   | 2241.5    |
| Small |  470.2    |

### BSON

| Size  | Time (µs) |
| ----- | --------- |
| Big   | 2260.5    |
| Small |  496.2    |

### CBOR

| Size  | Time (µs) |
| ----- | --------- |
| Big   | 1470.3    |
| Small |  333.5    |

### Bincode

| Size  | Time (µs) |
| ----- | --------- |
| Big   |  1042.0   |
| Small |   239.8   |

### Bitcode

| Size  | Time (µs) |
| ----- | --------- |
| Big   | 1752.8    |
| Small |  223.4    |

Not only YMMV, but also the results are not very consistent across runs. They depends a lot on the
system load (which can fluctuate a lot). So it's best to run the benchmarks multiple times and take
the average.

## Observations

- For small payloads, JSON performs competitively with binary formats.
- JSON encoding is most efficient when there are a lot of hashmaps involved, especially with string keys.
- Binary formats like Bincode and Bitcode show significant performance advantages for both payload sizes.
