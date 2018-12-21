use crate::algorithm::{Map, ITEMS};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::io::Write;

pub fn run(input: &[u8], mut out: impl Write) {
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .unwrap();

    // In reverse to spawn big tasks first
    let mut items: Vec<_> = ITEMS
        .iter()
        .rev()
        .map(|&item| (item, Map::default()))
        .collect();
    pool.install(|| {
        items.par_iter_mut().for_each(|(item, map)| {
            *map = item.gen_freq(input);
        });
    });

    for (item, future_freq) in items.into_iter().rev() {
        item.print(future_freq, &mut out);
    }
}
