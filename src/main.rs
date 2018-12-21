use rust_4::get_seq;

fn main() {
    let stdin = std::io::stdin();
    let input = get_seq(stdin.lock(), b">THREE");
    let mut out = std::io::stdout();

    rust_4::with_cpupool::run(&input, &mut out);
    rust_4::with_rayon::run(&input, &mut out);
    rust_4::with_tokiopool::run(&input, &mut out);
    rust_4::with_tokiopool_blocking::run(&input, &mut out);
    rust_4::with_scoped_threadpool::run(&input, &mut out);
}
