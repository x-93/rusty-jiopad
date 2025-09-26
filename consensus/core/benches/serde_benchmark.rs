use criterion::{black_box, criterion_group, criterion_main, Criterion};
use consensus_core::tx::{Transaction, TxInput, TxOutput};
use consensus_core::Hash;
use ciborium::{from_reader, ser};

fn create_transaction(num_inputs: usize, num_outputs: usize) -> Transaction {
    let inputs = (0..num_inputs)
        .map(|i| TxInput {
            prev_tx_hash: Hash::from_le_u64([i as u64, 2, 3, 4]),
            index: i as u32,
            script_sig: vec![0x00; 64], // Larger script for realism
            sequence: 0,
        })
        .collect();

    let outputs = (0..num_outputs)
        .map(|i| TxOutput {
            value: 100 + i as u64,
            script_pubkey: vec![0x76, 0xa9, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x88, 0xac],
        })
        .collect();

    Transaction::new(1, inputs, outputs, 0)
}

fn bench_transaction_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("transaction_serialization");

    // Small transaction (1 input, 1 output)
    let small_tx = create_transaction(1, 1);
    group.bench_function("json_serialize_small", |b| {
        b.iter(|| {
            let serialized = serde_json::to_string(black_box(&small_tx)).unwrap();
            black_box(serialized);
        });
    });
    group.bench_function("json_deserialize_small", |b| {
        let serialized = serde_json::to_string(&small_tx).unwrap();
        b.iter(|| {
            let deserialized: Transaction = serde_json::from_str(black_box(&serialized)).unwrap();
            black_box(deserialized);
        });
    });
    group.bench_function("cbor_serialize_small", |b| {
        b.iter(|| {
            let mut buffer = Vec::new();
            ser::into_writer(black_box(&small_tx), &mut buffer).unwrap();
            black_box(buffer);
        });
    });
    group.bench_function("cbor_deserialize_small", |b| {
        let mut buffer = Vec::new();
        ser::into_writer(&small_tx, &mut buffer).unwrap();
        b.iter(|| {
            let deserialized: Transaction = from_reader(black_box(&buffer[..])).unwrap();
            black_box(deserialized);
        });
    });

    // Medium transaction (5 inputs, 5 outputs)
    let medium_tx = create_transaction(5, 5);
    group.bench_function("json_serialize_medium", |b| {
        b.iter(|| {
            let serialized = serde_json::to_string(black_box(&medium_tx)).unwrap();
            black_box(serialized);
        });
    });
    group.bench_function("json_deserialize_medium", |b| {
        let serialized = serde_json::to_string(&medium_tx).unwrap();
        b.iter(|| {
            let deserialized: Transaction = serde_json::from_str(black_box(&serialized)).unwrap();
            black_box(deserialized);
        });
    });
    group.bench_function("cbor_serialize_medium", |b| {
        b.iter(|| {
            let mut buffer = Vec::new();
            ser::into_writer(black_box(&medium_tx), &mut buffer).unwrap();
            black_box(buffer);
        });
    });
    group.bench_function("cbor_deserialize_medium", |b| {
        let mut buffer = Vec::new();
        ser::into_writer(&medium_tx, &mut buffer).unwrap();
        b.iter(|| {
            let deserialized: Transaction = from_reader(black_box(&buffer[..])).unwrap();
            black_box(deserialized);
        });
    });

    // Large transaction (10 inputs, 10 outputs)
    let large_tx = create_transaction(10, 10);
    group.bench_function("json_serialize_large", |b| {
        b.iter(|| {
            let serialized = serde_json::to_string(black_box(&large_tx)).unwrap();
            black_box(serialized);
        });
    });
    group.bench_function("json_deserialize_large", |b| {
        let serialized = serde_json::to_string(&large_tx).unwrap();
        b.iter(|| {
            let deserialized: Transaction = serde_json::from_str(black_box(&serialized)).unwrap();
            black_box(deserialized);
        });
    });
    group.bench_function("cbor_serialize_large", |b| {
        b.iter(|| {
            let mut buffer = Vec::new();
            ser::into_writer(black_box(&large_tx), &mut buffer).unwrap();
            black_box(buffer);
        });
    });
    group.bench_function("cbor_deserialize_large", |b| {
        let mut buffer = Vec::new();
        ser::into_writer(&large_tx, &mut buffer).unwrap();
        b.iter(|| {
            let deserialized: Transaction = from_reader(black_box(&buffer[..])).unwrap();
            black_box(deserialized);
        });
    });

    group.finish();
}

fn bench_header_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("header_hashing");

    let header = consensus_core::header::Header::new();

    group.bench_function("header_hash", |b| {
        b.iter(|| {
            let hash = black_box(&header).hash();
            black_box(hash);
        });
    });

    group.bench_function("header_hash_with_nonce", |b| {
        b.iter(|| {
            let hash = black_box(&header).hash_with_nonce(black_box(12345));
            black_box(hash);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_transaction_serialization, bench_header_hashing);
criterion_main!(benches);
