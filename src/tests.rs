use crate::{BinaryWrite, BinaryRead};
use std::{fs::{OpenOptions}, io};


#[derive(Debug)]
struct Test {
    f: usize,
    d: char,
    g: u64
}

impl Test {
    fn new() -> Self {
        Self {
            f: 32,
            d: 'a',
            g: 13451234
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

    let s = Test::new();

    file.write(&s)?;

    Ok(())
}

#[test]
fn read_file() -> io::Result<()> {
    let mut file = OpenOptions::new()
    .read(true)
    .open("./test_file.bin")?;

    println!("Size {}", std::mem::size_of::<Test>());
    let a = file.read::<Test>()?;
    println!("Juan: {}", a.g);

    Ok(())
}
