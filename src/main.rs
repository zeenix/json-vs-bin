use json_vs_bin::{
    data::{BigData, SmallData},
    formats,
    vector_data::{BigVectorData, SmallVectorData},
};
use std::iter;

fn main() {
    println!("=== Encoded Size Analysis ===\n");

    // Create test data matching benchmarks
    let big_data = iter::repeat_with(BigData::new).take(10).collect::<Vec<_>>();
    let small_data = iter::repeat_with(SmallData::new)
        .take(10)
        .collect::<Vec<_>>();

    // Measure sizes for each format
    let json_big = formats::Json::encode_big(&big_data);
    let json_small = formats::Json::encode_small(&small_data);

    let simd_json_big = formats::SimdJson::encode_big(&big_data);
    let simd_json_small = formats::SimdJson::encode_small(&small_data);

    let dbus = formats::DBus::new();
    let dbus_big = dbus.encode_big(&big_data);
    let dbus_small = dbus.encode_small(&small_data);

    let bson_big = formats::Bson::encode_big(&big_data);
    let bson_small = formats::Bson::encode_small(&small_data);

    let cbor_big = formats::Cbor::encode_big(&big_data);
    let cbor_small = formats::Cbor::encode_small(&small_data);

    let bincode = formats::Bincode::new();
    let bincode_big = bincode.encode_big(&big_data);
    let bincode_small = bincode.encode_small(&small_data);

    let bitcode_big = formats::Bitcode::encode_big(&big_data);
    let bitcode_small = formats::Bitcode::encode_small(&small_data);

    // Vector data benchmarks
    let big_vector_data = iter::repeat_with(BigVectorData::new)
        .take(10)
        .collect::<Vec<_>>();
    let small_vector_data = iter::repeat_with(SmallVectorData::new)
        .take(10)
        .collect::<Vec<_>>();

    let json_big_vector = formats::Json::encode_big_vector(&big_vector_data);
    let json_small_vector = formats::Json::encode_small_vector(&small_vector_data);

    let simd_json_big_vector = formats::SimdJson::encode_big_vector(&big_vector_data);
    let simd_json_small_vector = formats::SimdJson::encode_small_vector(&small_vector_data);

    let dbus_big_vector = dbus.encode_big_vector(&big_vector_data);
    let dbus_small_vector = dbus.encode_small_vector(&small_vector_data);

    let bson_big_vector = formats::Bson::encode_big_vector(&big_vector_data);
    let bson_small_vector = formats::Bson::encode_small_vector(&small_vector_data);

    let cbor_big_vector = formats::Cbor::encode_big_vector(&big_vector_data);
    let cbor_small_vector = formats::Cbor::encode_small_vector(&small_vector_data);

    let bincode_big_vector = bincode.encode_big_vector(&big_vector_data);
    let bincode_small_vector = bincode.encode_small_vector(&small_vector_data);

    let bitcode_big_vector = formats::Bitcode::encode_big_vector(&big_vector_data);
    let bitcode_small_vector = formats::Bitcode::encode_small_vector(&small_vector_data);

    // Display results in a table
    println!("## HashMap-based Data (with long field names)");
    println!();
    println!("### Big Payload (10 instances)");
    println!();
    println!("| Format      | Size (bytes) | Ratio vs JSON |");
    println!("| ----------- | ------------ | ------------- |");
    print_row("JSON", json_big.len(), json_big.len());
    print_row("SIMD-JSON", simd_json_big.len(), json_big.len());
    print_row("D-Bus", dbus_big.len(), json_big.len());
    print_row("BSON", bson_big.len(), json_big.len());
    print_row("CBOR", cbor_big.len(), json_big.len());
    print_row("Bincode", bincode_big.len(), json_big.len());
    print_row("Bitcode", bitcode_big.len(), json_big.len());

    println!();
    println!("### Small Payload (10 instances)");
    println!();
    println!("| Format      | Size (bytes) | Ratio vs JSON |");
    println!("| ----------- | ------------ | ------------- |");
    print_row("JSON", json_small.len(), json_small.len());
    print_row("SIMD-JSON", simd_json_small.len(), json_small.len());
    print_row("D-Bus", dbus_small.len(), json_small.len());
    print_row("BSON", bson_small.len(), json_small.len());
    print_row("CBOR", cbor_small.len(), json_small.len());
    print_row("Bincode", bincode_small.len(), json_small.len());
    print_row("Bitcode", bitcode_small.len(), json_small.len());

    println!();
    println!("## Vector-based Data (arrays of structs)");
    println!();
    println!("### Big Vector Payload (10 instances)");
    println!();
    println!("| Format      | Size (bytes) | Ratio vs JSON |");
    println!("| ----------- | ------------ | ------------- |");
    print_row("JSON", json_big_vector.len(), json_big_vector.len());
    print_row(
        "SIMD-JSON",
        simd_json_big_vector.len(),
        json_big_vector.len(),
    );
    print_row("D-Bus", dbus_big_vector.len(), json_big_vector.len());
    print_row("BSON", bson_big_vector.len(), json_big_vector.len());
    print_row("CBOR", cbor_big_vector.len(), json_big_vector.len());
    print_row("Bincode", bincode_big_vector.len(), json_big_vector.len());
    print_row("Bitcode", bitcode_big_vector.len(), json_big_vector.len());

    println!();
    println!("### Small Vector Payload (10 instances)");
    println!();
    println!("| Format      | Size (bytes) | Ratio vs JSON |");
    println!("| ----------- | ------------ | ------------- |");
    print_row("JSON", json_small_vector.len(), json_small_vector.len());
    print_row(
        "SIMD-JSON",
        simd_json_small_vector.len(),
        json_small_vector.len(),
    );
    print_row("D-Bus", dbus_small_vector.len(), json_small_vector.len());
    print_row("BSON", bson_small_vector.len(), json_small_vector.len());
    print_row("CBOR", cbor_small_vector.len(), json_small_vector.len());
    print_row(
        "Bincode",
        bincode_small_vector.len(),
        json_small_vector.len(),
    );
    print_row(
        "Bitcode",
        bitcode_small_vector.len(),
        json_small_vector.len(),
    );

    println!();
    println!("## Summary");
    println!();
    println!("### HashMap-based data:");
    println!(
        "- JSON baseline sizes: {} bytes (big), {} bytes (small)",
        json_big.len(),
        json_small.len()
    );

    // Calculate compression ratios
    let formats = [
        ("SIMD-JSON", simd_json_big.len(), simd_json_small.len()),
        ("D-Bus", dbus_big.len(), dbus_small.len()),
        ("BSON", bson_big.len(), bson_small.len()),
        ("CBOR", cbor_big.len(), cbor_small.len()),
        ("Bincode", bincode_big.len(), bincode_small.len()),
        ("Bitcode", bitcode_big.len(), bitcode_small.len()),
    ];

    // Find the most compact format
    let most_compact_big = formats.iter().min_by_key(|(_, big, _)| big).unwrap();
    let most_compact_small = formats.iter().min_by_key(|(_, _, small)| small).unwrap();

    println!(
        "- Most compact for big payload: {} ({} bytes, {:.1}% of JSON size)",
        most_compact_big.0,
        most_compact_big.1,
        (most_compact_big.1 as f64 / json_big.len() as f64) * 100.0
    );
    println!(
        "- Most compact for small payload: {} ({} bytes, {:.1}% of JSON size)",
        most_compact_small.0,
        most_compact_small.2,
        (most_compact_small.2 as f64 / json_small.len() as f64) * 100.0
    );

    println!();
    println!("### Vector-based data:");
    println!(
        "- JSON baseline sizes: {} bytes (big), {} bytes (small)",
        json_big_vector.len(),
        json_small_vector.len()
    );

    // Calculate compression ratios for vector data
    let formats_vector = [
        (
            "SIMD-JSON",
            simd_json_big_vector.len(),
            simd_json_small_vector.len(),
        ),
        ("D-Bus", dbus_big_vector.len(), dbus_small_vector.len()),
        ("BSON", bson_big_vector.len(), bson_small_vector.len()),
        ("CBOR", cbor_big_vector.len(), cbor_small_vector.len()),
        (
            "Bincode",
            bincode_big_vector.len(),
            bincode_small_vector.len(),
        ),
        (
            "Bitcode",
            bitcode_big_vector.len(),
            bitcode_small_vector.len(),
        ),
    ];

    let most_compact_big_vector = formats_vector.iter().min_by_key(|(_, big, _)| big).unwrap();
    let most_compact_small_vector = formats_vector
        .iter()
        .min_by_key(|(_, _, small)| small)
        .unwrap();

    println!(
        "- Most compact for big payload: {} ({} bytes, {:.1}% of JSON size)",
        most_compact_big_vector.0,
        most_compact_big_vector.1,
        (most_compact_big_vector.1 as f64 / json_big_vector.len() as f64) * 100.0
    );
    println!(
        "- Most compact for small payload: {} ({} bytes, {:.1}% of JSON size)",
        most_compact_small_vector.0,
        most_compact_small_vector.2,
        (most_compact_small_vector.2 as f64 / json_small_vector.len() as f64) * 100.0
    );
}

fn print_row(format: &str, size: usize, json_size: usize) {
    let ratio = if json_size == 0 {
        0.0
    } else {
        (size as f64 / json_size as f64) * 100.0
    };

    if size == json_size {
        println!("| {:>11} | {:>12} | {:>12}% |", format, size, "100.0");
    } else {
        println!("| {:>11} | {:>12} | {:>12.1}% |", format, size, ratio);
    }
}
