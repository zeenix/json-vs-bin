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
| No                | Big   | 2013.8    |
| High (threads)    | Big   | 1867.4    |
| High (tasks)      | Big   | 1720.5    |
| No                | Small |  441.4    |
| High (threads)    | Small |  439.1    |
| High (tasks)      | Small |  360.6    |


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
| No                | Big   | 2260.5    |
| High (threads)    | Big   | 2131.9    |
| High (tasks)      | Big   | 1968.5    |
| No                | Small |  496.2    |
| High (threads)    | Small |  509.0    |
| High (tasks)      | Small |  415.7    |

### CBOR

| Context Switching | Size  | Time (µs) |
| ----------------- | ----- | --------- |
| No                | Big   | 1470.3    |
| High (threads)    | Big   | 1241.7    |
| High (tasks)      | Big   | 1145.4    |
| No                | Small |  333.5    |
| High (threads)    | Small |  329.0    |
| High (tasks)      | Small |  245.2    |

### Bincode 

| Context Switching | Size  | Time (µs) |
| ----------------- | ----- | --------- |
| No                | Big   |  1042.0   |
| High (threads)    | Big   |   779.7   |
| High (tasks)      | Big   |   688.2   |
| No                | Small |   239.8   |
| High (threads)    | Small |   227.6   |
| High (tasks)      | Small |   152.2   |

### Bitcode

| Context Switching | Size  | Time (µs) |
| ----------------- | ----- | --------- |
| No                | Big   | 1752.8    |
| High (threads)    | Big   | 1942.5    |
| High (tasks)      | Big   |  619.2    |
| No                | Small |  223.4    |
| High (threads)    | Small |  207.7    |
| High (tasks)      | Small |  139.2    |

Not only YMMV, but also the results are not very consistent across runs. They depends a lot on the
system load (which can fluctuate a lot). So it's best to run the benchmarks multiple times and take
the average.

## Observations

- For small payloads, JSON is at least as fast as the binary formats.
- JSON encoding is most efficient when there are a lot of hashmaps involved, especially with string keys.
- Effect of context switching is more pronounced when the payload is small.
- Tokio tasks (backed by thread pools) are significantly more efficient than pure threads.
