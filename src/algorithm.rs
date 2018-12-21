// The following code has been taken from the following web page:
// https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/knucleotide-rust-4.html
// .. with some small amends.
//
// The Computer Language Benchmarks Game
// https://salsa.debian.org/benchmarksgame-team/benchmarksgame/
//
// contributed by the Rust Project Developers
// contributed by Cristi Cobzarenco (@cristicbz)
// contributed by TeXitoi

use self::Item::*;
use indexmap::IndexMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::io::Write;

pub struct NaiveHasher(u64);
impl Default for NaiveHasher {
    fn default() -> Self {
        NaiveHasher(0)
    }
}

impl Hasher for NaiveHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, _: &[u8]) {
        unimplemented!()
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = i ^ i >> 7;
    }
}

type NaiveBuildHasher = BuildHasherDefault<NaiveHasher>;
type NaiveHashMap<K, V> = IndexMap<K, V, NaiveBuildHasher>;
pub type Map = NaiveHashMap<Code, u32>;

#[derive(Hash, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub struct Code(u64);
impl Code {
    fn push(&mut self, c: u8, mask: u64) {
        self.0 <<= 2;
        self.0 |= u64::from(c);
        self.0 &= mask;
    }
    fn from_str(s: &str) -> Code {
        let mask = Code::make_mask(s.len());
        let mut res = Code(0);
        for c in s.as_bytes() {
            res.push(Code::encode(*c), mask);
        }
        res
    }
    fn to_string(self, frame: usize) -> String {
        let mut res = vec![];
        let mut code = self.0;
        for _ in 0..frame {
            let c = match code as u8 & 0b11 {
                c if c == Code::encode(b'A') => b'A',
                c if c == Code::encode(b'T') => b'T',
                c if c == Code::encode(b'G') => b'G',
                c if c == Code::encode(b'C') => b'C',
                _ => unreachable!(),
            };
            res.push(c);
            code >>= 2;
        }
        res.reverse();
        String::from_utf8(res).unwrap()
    }
    fn make_mask(frame: usize) -> u64 {
        (1u64 << (2 * frame)) - 1
    }
    fn encode(c: u8) -> u8 {
        (c & 0b110) >> 1
    }
}

struct Iter<'a> {
    iter: std::slice::Iter<'a, u8>,
    code: Code,
    mask: u64,
}

impl<'a> Iter<'a> {
    fn new(input: &[u8], frame: usize) -> Iter {
        let mut iter = input.iter();
        let mut code = Code(0);
        let mask = Code::make_mask(frame);
        for c in iter.by_ref().take(frame - 1) {
            code.push(*c, mask);
        }
        Iter { iter, code, mask }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Code;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|&c| {
            self.code.push(c, self.mask);
            self.code
        })
    }
}

pub fn gen_freq(input: &[u8], frame: usize) -> Map {
    let mut freq = Map::default();
    for code in Iter::new(input, frame) {
        *freq.entry(code).or_insert(0) += 1;
    }
    freq
}

#[derive(Clone, Copy)]
pub enum Item {
    Freq(usize),
    Occ(&'static str),
}
impl Item {
    pub fn print(self, freq: Map, mut out: impl Write) {
        match self {
            Freq(frame) => {
                let mut v: Vec<_> = freq
                    .into_iter()
                    .map(|(code, count)| (count, code))
                    .collect();
                v.sort();
                let total = v.iter().map(|&(count, _)| count).sum::<u32>() as f32;
                for (count, key) in v.into_iter().rev() {
                    writeln!(
                        out,
                        "{} {:.3}",
                        key.to_string(frame),
                        (count as f32 * 100.) / total
                    )
                    .unwrap();
                }
                writeln!(out).unwrap();
            }
            Occ(occ) => writeln!(out, "{}\t{}", freq[&Code::from_str(occ)], occ).unwrap(),
        }
    }
    pub fn gen_freq(&self, input: &[u8]) -> Map {
        match *self {
            Freq(frame) => gen_freq(input, frame),
            Occ(occ) => gen_freq(input, occ.len()),
        }
    }
}
pub static ITEMS: [Item; 7] = [
    Freq(1),
    Freq(2),
    Occ("GGT"),
    Occ("GGTA"),
    Occ("GGTATT"),
    Occ("GGTATTTTAATT"),
    Occ("GGTATTTTAATTTATAGT"),
];

pub fn get_seq<R: std::io::BufRead>(mut r: R, key: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(65536);
    let mut line = Vec::with_capacity(64);

    loop {
        match r.read_until(b'\n', &mut line) {
            Ok(b) if b > 0 => {
                if line.starts_with(key) {
                    break;
                }
            }
            _ => break,
        }
        line.clear();
    }

    loop {
        line.clear();
        match r.read_until(b'\n', &mut line) {
            Ok(b) if b > 0 => res.extend(line[..line.len() - 1].iter().cloned().map(Code::encode)),
            _ => break,
        }
    }

    res
}
