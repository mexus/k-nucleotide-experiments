mod algorithm;
pub use self::algorithm::{get_seq, Code, Item, Map, ITEMS};
pub mod with_cpupool;
pub mod with_rayon;
pub mod with_scoped_threadpool;
pub mod with_tokiopool;
pub mod with_tokiopool_blocking;
