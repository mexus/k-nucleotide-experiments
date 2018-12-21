use crate::algorithm::{Map, ITEMS};
use scoped_threadpool::Pool;
use std::io::Write;

pub fn run(input: &[u8], mut out: impl Write) {
    let mut pool = Pool::new(num_cpus::get() as u32);
    // In reverse to spawn big tasks first
    let mut items: Vec<_> = ITEMS
        .iter()
        .rev()
        .map(|&item| (item, Map::default()))
        .collect();
    pool.scoped(|scoped| {
        for (item, map) in &mut items {
            scoped.execute(move || *map = item.gen_freq(input));
        }
    });

    for (item, future_freq) in items.into_iter().rev() {
        item.print(future_freq, &mut out);
    }
}
