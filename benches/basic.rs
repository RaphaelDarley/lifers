use std::iter::repeat_with;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lifers::{basic_swap, game1, Game};
use rand::Rng;

fn nticks(game: &impl Game, n: usize) {
    let mut game = game.to_owned();
    for _ in 0..n {
        game.tick();
    }
}

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

fn bench_games(c: &mut Criterion) {
    {
        let small: Vec<Vec<bool>> = vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ];
        let mut group = c.benchmark_group("small");

        group.bench_with_input(
            BenchmarkId::new("game1", "blinker"),
            &game1::State::from(small.clone()),
            |b, g| b.iter(|| nticks(g, 100)),
        );
        group.bench_with_input(
            BenchmarkId::new("basic_swap", "blinker"),
            &basic_swap::State::from(small.clone()),
            |b, g| b.iter(|| nticks(g, 100)),
        );
        group.finish();
    }
    {
        let mut rng = rand::thread_rng();

        let large = repeat_with(|| {
            repeat_with(|| rng.gen_bool(0.5))
                .take(1000)
                .collect::<Vec<bool>>()
        })
        .take(1000)
        .collect::<Vec<Vec<bool>>>();

        let mut group = c.benchmark_group("large");
        group.sample_size(10);

        group.bench_with_input(
            BenchmarkId::new("game1", "large random, 10 ticks"),
            &game1::State::from(large.clone()),
            |b, g| b.iter(|| nticks(g, 10)),
        );
        group.bench_with_input(
            BenchmarkId::new("game1", "large random, 100 ticks"),
            &game1::State::from(large.clone()),
            |b, g| b.iter(|| nticks(g, 100)),
        );
        group.bench_with_input(
            BenchmarkId::new("basic_swap", "large random, 10 ticks"),
            &basic_swap::State::from(large.clone()),
            |b, g| b.iter(|| nticks(g, 10)),
        );
        group.bench_with_input(
            BenchmarkId::new("basic_swap", "large random, 100 ticks"),
            &basic_swap::State::from(large.clone()),
            |b, g| b.iter(|| nticks(g, 100)),
        );
        group.finish();
    }
}

criterion_group!(benches, bench_games);
criterion_main!(benches);
