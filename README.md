# Benchmarking JSON vs. binary formats

This repo benchmarks performance of JSON vs. binary format encoding using serde. We take into
account the effects of context switching and size of the payload on the performance of the encoding
and decoding.

## Running the benchmarks

```bash
cargo +nightly bench
```

## Results

The results on my machines (from one of the runs) are as follows:

| Format | Context Switching | Size  | Time (µs) |
| ------ | ----------------- | ----- | --------- |
| JSON   | No                | Big   | 2711.3    |
| JSON   | High              | Big   | 3655.1    |
| JSON   | No                | Small | 527.88    |
| JSON   | High              | Small | 907.90    |
| D-Bus  | No                | Big   | 2241.5    |
| D-Bus  | High              | Big   | 3163.9    |
| D-Bus  | No                | Small | 470.29    |
| D-Bus  | High              | Small | 1964.9    |
| BSON   | No                | Big   | 2296.8    |
| BSON   | High              | Big   | 3154.1    |
| BSON   | No                | Small | 439.50    |
| BSON   | High              | Small | 1444.4    |
| CBOR   | No                | Big   | 1441.8    |
| CBOR   | High              | Big   | 2420.1    |
| CBOR   | No                | Small | 273.66    |
| CBOR   | High              | Small | 1212.1    |

Not only YMMV, but also the results are not very consistent across runs. They depends a lot on the
system load (which can fluctuate a lot). So it's best to run the benchmarks multiple times and take
the average.

## Observations

- For small payloads, JSON is at least as fast as the binary formats.
- JSON encoding is most efficient when there are a lot of hashmaps invovled, especially with string keys.
- Effect of context switching is more pronounced when the payload is small.
