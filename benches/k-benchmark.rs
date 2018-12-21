use criterion::{criterion_group, criterion_main, Bencher, Criterion, Fun};
use rust_4::{
    get_seq, with_cpupool, with_rayon, with_scoped_threadpool, with_tokiopool,
    with_tokiopool_blocking,
};
use std::fs::File;
use std::io::{BufReader, Write};
use std::time::Duration;

struct Null;
impl Write for Null {
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        Ok(())
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let cpupool = Fun::new("futures-cpupool", |b: &mut Bencher, i: &Vec<u8>| {
        b.iter(|| with_cpupool::run(i, Null))
    });
    let rayon = Fun::new("rayon", |b: &mut Bencher, i: &Vec<u8>| {
        b.iter(|| with_rayon::run(i, Null))
    });
    let tokiopool = Fun::new("tokio-threadpool", |b: &mut Bencher, i: &Vec<u8>| {
        b.iter(|| with_tokiopool::run(i, Null))
    });
    let tokiopool_blocking = Fun::new(
        "tokio-threadpool (blocking)",
        |b: &mut Bencher, i: &Vec<u8>| b.iter(|| with_tokiopool_blocking::run(i, Null)),
    );
    let scoped_threadpool = Fun::new("scoped_threadpool", |b: &mut Bencher, i: &Vec<u8>| {
        b.iter(|| with_scoped_threadpool::run(i, Null))
    });
    let functions = vec![
        cpupool,
        rayon,
        tokiopool,
        tokiopool_blocking,
        scoped_threadpool,
    ];

    let input = File::open("benches/input.txt").unwrap();
    let bytes = get_seq(BufReader::new(input), b">THREE");
    // let input = include_bytes!("input.txt");
    // let bytes = get_seq(BufReader::new(&input[..]), b">THREE");
    c.bench_functions("K-nucleotide", functions, bytes);
}

criterion_group!(
    name = benches;
    config = Criterion::default()
             .warm_up_time(Duration::from_secs(15))
             .sample_size(5);
    targets = criterion_benchmark
);
criterion_main!(benches);
