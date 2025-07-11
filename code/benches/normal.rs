use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use greed::GreedSolver;

fn normal_states(c: &mut Criterion) {
    let mut group = c.benchmark_group("normal_states");

    const RULESETS: [(u32, u32); 3] = [(25, 4), (100, 6), (250, 20)];

    for ruleset in RULESETS {
        // satisfy invariants
        let mut solver = GreedSolver::new(ruleset.0, ruleset.1);
        solver.solve();

        // Benchmark: solving normal states
        group.bench_with_input(
            BenchmarkId::new("solve", format!("M={},s={}", ruleset.0, ruleset.1)),
            &ruleset,
            |b, _| {
                b.iter(|| solver.solve_normal_states());
            },
        );

        // Benchmark: find optimal action
        group.bench_with_input(
            BenchmarkId::new(
                "calc_optimal_payoff",
                format!("M={},s={}", ruleset.0, ruleset.1),
            ),
            &ruleset,
            |b, _| {
                b.iter(|| {
                    solver.find_optimal_normal_action(black_box(greed::State::new(10, 10, false)))
                });
            },
        );

        // Benchmark: computing an optimal payoff
        group.bench_with_input(
            BenchmarkId::new("calc_payoff", format!("M={},s={}", ruleset.0, ruleset.1)),
            &ruleset,
            |b, _| {
                b.iter(|| {
                    solver.calc_normal_payoff(
                        black_box(greed::State::new(ruleset.0 / 2, ruleset.1 / 2, false)),
                        3,
                    )
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, normal_states);
criterion_main!(benches);
