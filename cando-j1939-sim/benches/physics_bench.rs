use criterion::{Criterion, black_box, criterion_group, criterion_main};

use cando_j1939_sim::SimulatorState;

fn bench_update_physics(c: &mut Criterion) {
    let mut state = SimulatorState::default();
    // Pre-warm with some realistic values
    state.engine.engine_speed = 1500.0;
    state.engine.engine_load = 50.0;

    c.bench_function("update_physics", |b| {
        b.iter(|| {
            let mut s = black_box(state.clone());
            s.update_physics(black_box(0.01)); // 10ms tick
        })
    });
}

fn bench_generate_can_frames(c: &mut Criterion) {
    let mut state = SimulatorState::default();
    state.engine.engine_speed = 1500.0;
    state.engine.engine_load = 50.0;
    state.update_physics(0.1);

    c.bench_function("generate_can_frames", |b| {
        b.iter(|| {
            let frames = black_box(&state).generate_can_frames();
            black_box(frames);
        })
    });
}

fn bench_full_broadcast_cycle(c: &mut Criterion) {
    c.bench_function("full_broadcast_cycle", |b| {
        b.iter(|| {
            let mut state = SimulatorState::default();
            state.engine.engine_speed = 1500.0;
            state.engine.engine_load = 50.0;
            state.update_physics(black_box(0.01));
            let frames = state.generate_can_frames();
            black_box(frames);
        })
    });
}

criterion_group!(
    benches,
    bench_update_physics,
    bench_generate_can_frames,
    bench_full_broadcast_cycle,
);
criterion_main!(benches);
