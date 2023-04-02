use std::io::Cursor;
use criterion::{Bencher, Criterion, criterion_group};
use binext::{BinaryRead, BinaryWrite};
use crate::benchmarks::{fill, FromArray};

#[allow(unused)]
#[derive(Debug)]
struct WithoutArray {
    a: char,
    b: u16,
    c: i16,
    d: u32,
    e: u8,
    f: i128,
    g: u128
}

fn without_array(b: &mut Bencher) {
    let mut buf = Vec::new();

    b.iter(move || {
        let item = WithoutArray {
            a: rand::random(),
            b: rand::random(),
            c: rand::random(),
            d: rand::random(),
            e: rand::random(),
            f: rand::random(),
            g: rand::random()
        };

        buf.write_binary(&item).unwrap();
        Cursor::new(&mut buf).read_binary::<WithoutArray>().unwrap();
    });
}

#[allow(unused)]
#[derive(Debug)]
struct WithArray {
    a: char,
    b: u16,
    c: i16,
    d: u32,
    e: u8,
    f: i128,
    g: u128,
    arr: [i64; 2048]
}

impl FromArray<i64, (), 2048> for WithArray {
    fn from_array(arr: [i64; 2048], _: ()) -> Self {
        WithArray {
            a: rand::random(),
            b: rand::random(),
            c: rand::random(),
            d: rand::random(),
            e: rand::random(),
            f: rand::random(),
            g: rand::random(),
            arr
        }
    }
}

fn with_array(b: &mut Bencher) {
    let mut buf = Vec::new();

    b.iter(move || {
        let item = fill::<WithArray, i64, (), 2048>(());
        buf.write_binary(&item).unwrap();
        Cursor::new(&mut buf).read_binary::<WithArray>().unwrap();
    });
}

pub fn bench_group(c: &mut Criterion) {
    c.bench_function("Without array", without_array);
    c.bench_function("With array", with_array);
}

criterion_group!(multiple_fields, bench_group);
