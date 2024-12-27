#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use octaver::Octaver;
use utils::generate_signal_stream;

fn octaver_bench(c: &mut Criterion) {
  let mut octaver = Octaver::new(44100.);
  let mut params = Params::new(44100.);
  params.set(1., 0.75);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("octaver", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        octaver.process(*signal, &mut params);
      }
    })
  });
}

criterion_group!(benches, octaver_bench);
criterion_main!(benches);
