use crate::{BinaryWrite, BinaryRead};
use std::{fs::{OpenOptions}, io::{self}};

#[derive(Debug, PartialEq, Eq)]
#[allow(unused)]
struct Test {
    f: u32,
    d: usize,
    g: u8
}

impl Test {
    fn random() -> Self {
        Self {
            f: rand::random(),
            d: rand::random(),
            g: rand::random()
        }
    }
}

#[test]
fn write_file() -> io::Result<()> {
    let mut file = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open("./test_file.bin")?;

    let s = Test::random();

    file.write_binary(&s)?;
    println!("{s:?}");

    Ok(())
}

#[test]
fn read_file() -> io::Result<()> {
    let mut file = OpenOptions::new()
    .read(true)
    .open("./test_file2.bin")?;

    let a = file.read_binary::<Test>()?;
    println!("{a:?}");

    Ok(())
}

fn write_buffer() -> io::Result<(Test, io::Cursor<Vec<u8>>)> {
    let test = Test::random();
    let mut buf = Vec::new();

    buf.write_binary(&test)?;

    Ok((test, io::Cursor::new(buf)))
}

#[test]
fn read_write_buffer() -> io::Result<()> {
    let (original, mut buf) = write_buffer()?;
    let test = buf.read_binary::<Test>()?;
    
    assert_eq!(original, test);
    Ok(())
}
