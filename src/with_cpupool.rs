use crate::algorithm::ITEMS;
use futures::Future;
use futures_cpupool::CpuPool;
use std::io::Write;
use std::sync::Arc;

pub fn run(input: &[u8], mut out: impl Write) {
    let pool = CpuPool::new_num_cpus();
    let input: Arc<[u8]> = Arc::from(input);

    // In reverse to spawn big tasks first
    let items: Vec<_> = ITEMS
        .iter()
        .rev()
        .map(|&item| {
            let input = input.clone();
            let future_freq = pool.spawn_fn(move || Ok::<_, ()>(item.gen_freq(&input)));
            (item, future_freq)
        })
        .collect();

    for (item, future_freq) in items.into_iter().rev() {
        item.print(future_freq.wait().unwrap(), &mut out);
    }
}
