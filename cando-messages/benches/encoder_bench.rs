use criterion::{Criterion, black_box, criterion_group, criterion_main};

use cando_messages::common::DeviceId;
use cando_messages::encoder::{
    apply_inverse_scaling, apply_scaling, embed_device_id, extract_device_id, extract_signal,
    pack_signal,
};
use cando_messages::j1939::EEC1;

// ============================================================================
// Signal Extraction / Packing
// ============================================================================

fn bench_extract_signal(c: &mut Criterion) {
    let data = [0xA7, 0x2E, 0x00, 0x7D, 0xFF, 0x00, 0x8C, 0xF0];

    let mut group = c.benchmark_group("extract_signal");
    group.bench_function("8bit", |b| {
        b.iter(|| extract_signal(black_box(&data), 0, 8))
    });
    group.bench_function("16bit", |b| {
        b.iter(|| extract_signal(black_box(&data), 8, 16))
    });
    group.bench_function("32bit", |b| {
        b.iter(|| extract_signal(black_box(&data), 0, 32))
    });
    group.bench_function("2bit", |b| {
        b.iter(|| extract_signal(black_box(&data), 4, 2))
    });
    group.finish();
}

fn bench_pack_signal(c: &mut Criterion) {
    let mut group = c.benchmark_group("pack_signal");
    group.bench_function("8bit", |b| {
        b.iter(|| {
            let mut data = [0u8; 8];
            pack_signal(black_box(&mut data), 0, 8, 0xA7)
        })
    });
    group.bench_function("16bit", |b| {
        b.iter(|| {
            let mut data = [0u8; 8];
            pack_signal(black_box(&mut data), 8, 16, 0x2EA7)
        })
    });
    group.bench_function("32bit", |b| {
        b.iter(|| {
            let mut data = [0u8; 8];
            pack_signal(black_box(&mut data), 0, 32, 0x7D002EA7)
        })
    });
    group.finish();
}

// ============================================================================
// Scaling
// ============================================================================

fn bench_apply_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("apply_scaling");
    // Temperature: (1, -40) unsigned 8-bit
    group.bench_function("unsigned_8bit", |b| {
        b.iter(|| apply_scaling(black_box(65), 1.0, -40.0, false, 8))
    });
    // Engine speed: (0.125, 0) unsigned 16-bit
    group.bench_function("unsigned_16bit", |b| {
        b.iter(|| apply_scaling(black_box(12000), 0.125, 0.0, false, 16))
    });
    // Signed torque: (1, -125) signed 8-bit
    group.bench_function("signed_8bit", |b| {
        b.iter(|| apply_scaling(black_box(200), 1.0, -125.0, true, 8))
    });
    group.finish();
}

fn bench_apply_inverse_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("apply_inverse_scaling");
    group.bench_function("unsigned_8bit", |b| {
        b.iter(|| apply_inverse_scaling(black_box(25.0), 1.0, -40.0, 8))
    });
    group.bench_function("unsigned_16bit", |b| {
        b.iter(|| apply_inverse_scaling(black_box(1500.0), 0.125, 0.0, 16))
    });
    group.finish();
}

// ============================================================================
// Device ID
// ============================================================================

fn bench_device_id(c: &mut Criterion) {
    let mut group = c.benchmark_group("device_id");
    // PDU2 extraction
    group.bench_function("extract_pdu2", |b| {
        b.iter(|| extract_device_id(black_box(0x18F37082)))
    });
    // PDU1 extraction
    group.bench_function("extract_pdu1", |b| {
        b.iter(|| extract_device_id(black_box(0x187D8202)))
    });
    // PDU2 embed
    group.bench_function("embed_pdu2", |b| {
        b.iter(|| embed_device_id(black_box(0x18F37000), DeviceId::from(0x82), None))
    });
    // PDU1 embed
    group.bench_function("embed_pdu1", |b| {
        b.iter(|| {
            embed_device_id(
                black_box(0x187D0000),
                DeviceId::from(0x82),
                Some(0x02),
            )
        })
    });
    group.finish();
}

// ============================================================================
// Full Message Encode/Decode
// ============================================================================

fn bench_eec1_roundtrip(c: &mut Criterion) {
    let msg = EEC1 {
        device_id: DeviceId::from(0x42),
        engine_torque_mode: 3,
        atl_engn_prnt_trq_frtnl: 0.5,
        drvr_s_dmnd_engn_prnt_trq: 75.0,
        actual_engine_percent_torque: 80.0,
        engine_speed: 1500.0,
        sr_addrss_of_cntrllng_dv_fr_engn_cntrl: 0,
        engine_starter_mode: 0,
        engine_demand_percent_torque: 70.0,
    };

    let mut group = c.benchmark_group("eec1");
    group.bench_function("encode", |b| {
        b.iter(|| black_box(&msg).encode().unwrap())
    });

    let (can_id, data) = msg.encode().unwrap();
    group.bench_function("decode", |b| {
        b.iter(|| EEC1::decode(black_box(can_id), black_box(&data)))
    });

    group.bench_function("roundtrip", |b| {
        b.iter(|| {
            let (id, data) = black_box(&msg).encode().unwrap();
            EEC1::decode(id, &data)
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_extract_signal,
    bench_pack_signal,
    bench_apply_scaling,
    bench_apply_inverse_scaling,
    bench_device_id,
    bench_eec1_roundtrip,
);
criterion_main!(benches);
