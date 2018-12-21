use crate::algorithm::ITEMS;
use futures::future::lazy;
use futures::Future;
use std::io::Write;
use std::sync::Arc;
use tokio_threadpool::Builder;

pub fn run(input: &[u8], mut out: impl Write) {
    let input: Arc<[u8]> = Arc::from(input);
    let pool = Builder::new().pool_size(num_cpus::get()).build();
    // In reverse to spawn big tasks first
    let items: Vec<_> = ITEMS
        .iter()
        .rev()
        .map(|&item| {
            let input = input.clone();
            let future_freq = pool.spawn_handle(lazy(move || Ok::<_, ()>(item.gen_freq(&input))));
            (item, future_freq)
        })
        .collect();

    for (item, future_freq) in items.into_iter().rev() {
        item.print(future_freq.wait().unwrap(), &mut out);
    }
}
