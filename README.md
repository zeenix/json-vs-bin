# Benchmarking JSON vs. binary formats

This repo benchmarks performance of JSON vs. binary format encoding using serde. We take into
account the effects of context switching and size of the payload on the performance of the encoding
and decoding.

## Running the benchmarks

```bash
cargo +nightly bench
```

## Results

The results on my machines (from one of the runs) are as follows for different formats:

### JSON

| Context Switching | Size  | Time (µs) |
| ----------------- | ----- | --------- |
| No                | Big   | 2711.3    |
| High (threads)    | Big   | 3655.1    |
| High (tasks)      | Big   | 3029.0    |
| No                | Small |  527.8    |
| High (threads)    | Small |  907.9    |
| High (tasks)      | Small |  605.5    |

### D-Bus

| Context Switching | Size  | Time (µs) |
| ----------------- | ----- | --------- |
| No                | Big   | 2241.5    |
| High (threads)    | Big   | 3163.9    |
| High (tasks)      | Big   | 2599.3    |
| No                | Small |  470.2    |
| High (threads)    | Small | 1964.9    |
| High (tasks)      | Small |  541.7    |

### BSON

| Context Switching | Size  | Time (µs) |
| ----------------- | ----- | --------- |
| No                | Big   | 2296.8    |
| High (threads)    | Big   | 3154.1    |
| High (tasks)      | Big   | 2636.9    |
| No                | Small |  439.5    |
| High (threads)    | Small | 1444.4    |
| High (tasks)      | Small |  507.5    |

### CBOR

| Context Switching | Size  | Time (µs) |
| ----------------- | ----- | --------- |
| No                | Big   | 1441.8    |
| High (threads)    | Big   | 2420.1    |
| High (tasks)      | Big   | 1766.7    |
| No                | Small |  273.6    |
| High (threads)    | Small | 1212.1    |
| High (tasks)      | Small |  319.0    |

### Bincode 

| Context Switching | Size  | Time (µs) |
| ----------------- | ----- | --------- |
| No                | Big   | 2096.4    |
| High (threads)    | Big   | 3139.1    |
| High (tasks)      | Big   | 2630.9    |
| No                | Small |  142.7    |
| High (threads)    | Small |  806.2    |
| High (tasks)      | Small |  183.1    |

Not only YMMV, but also the results are not very consistent across runs. They depends a lot on the
system load (which can fluctuate a lot). So it's best to run the benchmarks multiple times and take
the average.

## Observations

- For small payloads, JSON is at least as fast as the binary formats.
- JSON encoding is most efficient when there are a lot of hashmaps involved, especially with string keys.
- Effect of context switching is more pronounced when the payload is small.
- Tokio tasks (backed by thread pools) are significantly more efficient than pure threads.
