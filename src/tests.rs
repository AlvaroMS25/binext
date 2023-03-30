use crate::{BinaryWrite, BinaryRead};
use std::{fs::{File, OpenOptions}, io, collections::binary_heap};


#[derive(Debug)]
struct AA {
    f: usize,
    d: char,
    g: u64
}

impl AA {
    fn new() -> Self {
        Self {
            f: 32,
            d: 'a',
            g: 13451234
        }
    }
}

#[test]
fn size() {
    println!("Size: {}", std::mem::size_of::<AA>());
}

#[test]
fn test_write() -> io::Result<()> {
    let mut file = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open("./test_file.bin")?;

    let s = AA::new();

    file.write(&s)?;

    Ok(())
}

#[test]
fn test_read() -> io::Result<()> {
    let mut file = OpenOptions::new()
    .read(true)
    .open("./test_file.bin")?;

    println!("Size {}", std::mem::size_of::<AA>());
    let a = file.read::<AA>()?;
    println!("Juan: {}", a.g);

    Ok(())
}
