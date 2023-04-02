use std::io::{Cursor, Read, Write};
use criterion::{Bencher, black_box, Criterion, criterion_group};
use rand::Rng;
use binext::{BinaryRead, BinaryWrite};
use super::{FromArray, fill};

struct Buf8192 {
    buffer: [char; 8192]
}

impl FromArray<char, (), 8192> for Buf8192 {
    fn from_array(array: [char; 8192], _: ()) -> Self {
        Self {
            buffer: array
        }
    }
}

pub fn buf_8192(b: &mut Bencher) {
    let mut buf = Vec::with_capacity(8192);

    b.iter(move || {
        let item = fill::<Buf8192, char, (), 8192>(());
        buf.write_binary(&item).unwrap();
        Cursor::new(&mut buf).read_binary::<Buf8192>().unwrap();
    })
}

struct Buf65535 {
    buffer: [char; 65535]
}

impl FromArray<char, (), 65535> for Buf65535 {
    fn from_array(array: [char; 65535], _: ()) -> Self {
        Self {
            buffer: array
        }
    }
}

pub fn buf_65535(b: &mut Bencher) {
    let mut buf = Vec::with_capacity(65535);

    b.iter(move || {
        let item = fill::<Buf65535, char, (), 65535>(());
        buf.write_binary(&item).unwrap();
        Cursor::new(&mut buf).read_binary::<Buf65535>().unwrap();
    })
}

fn bench_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Single buffer");

    group.bench_function("8192", buf_8192);
    group.bench_function("65535", buf_65535);
}

criterion_group!(single_buffer, bench_group);

