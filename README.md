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
| JSON   | High (threads)    | Big   | 3655.1    |
| JSON   | High (tasks)      | Big   | 3029.0    |
| JSON   | No                | Small |  527.8    |
| JSON   | High (threads)    | Small |  907.9    |
| JSON   | High (tasks)      | Small |  605.5    |
| D-Bus  | No                | Big   | 2241.5    |
| D-Bus  | High (threads)    | Big   | 3163.9    |
| D-Bus  | High (tasks)      | Big   | 2599.3    |
| D-Bus  | No                | Small |  470.2    |
| D-Bus  | High (threads)    | Small | 1964.9    |
| D-Bus  | High (tasks)      | Small |  541.7    |
| BSON   | No                | Big   | 2296.8    |
| BSON   | High (threads)    | Big   | 3154.1    |
| BSON   | High (tasks)      | Big   | 2636.9    |
| BSON   | No                | Small |  439.5    |
| BSON   | High (threads)    | Small | 1444.4    |
| BSON   | High (tasks)      | Small |  507.5    |
| CBOR   | No                | Big   | 1441.8    |
| CBOR   | High (threads)    | Big   | 2420.1    |
| CBOR   | High (tasks)      | Big   | 1766.7    |
| CBOR   | No                | Small |  273.6    |
| CBOR   | High (threads)    | Small | 1212.1    |
| CBOR   | High (tasks)      | Small |  319.0    |

Not only YMMV, but also the results are not very consistent across runs. They depends a lot on the
system load (which can fluctuate a lot). So it's best to run the benchmarks multiple times and take
the average.

## Observations

- For small payloads, JSON is at least as fast as the binary formats.
- JSON encoding is most efficient when there are a lot of hashmaps invovled, especially with string keys.
- Effect of context switching is more pronounced when the payload is small.
- Tokio tasks (backed by thread pools) are significantly more efficient than pure threads.
