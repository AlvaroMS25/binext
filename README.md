# binext

binext is a library that aims to make easier working with binary buffers and structs like you would in C.

This library provides safe interfaces to write/read structs from [Read]/[Write] sources.

If used along with `#[repr(C)]`, this crate allows to read/write binary structures between C/C++ and Rust.

---

## Motivation

I found interesting the way C/C++ allows to write/read a struct from a binary file just by casting a pointer
of it to a `char*`. I wanted to know if rust had any std implementation for that, or if there was a crate that
allowed me to do that, but i couldn't find any. So i just had to implement my own one! And here we are now!

# Examples

## Pure rust
Reading / writing to files from rust is pretty easy, let's take a look at an example:

```rust
use binext::{BinaryRead, BinaryWrite};
use std::{io, fs::OpenOptions};

#[derive(Debug)]
struct MyStruct {
    a: usize,
    b: char
}

fn main() -> io::Result<()> {
    // Open a file to write to.
    let mut write_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("myfile.bin")?;

    let my_struct = MyStruct { a: 128, b: 'a' };
    // Write the struct instance data into the file.
    write_file.write_binary(&my_struct)?;

    drop(write_file);
    drop(my_struct);


    // Now open the same file but in read only mode.
    let mut read_file = OpenOptions::new().read(true).open("myfile.bin")?;

    // This will return a struct with the same data as the instance used to write
    // a: 128, b: 'a'
    let out = read_file.read_binary::<MyStruct>()?;
    println!("{out:?}");

    Ok(())
}
```

## C/C++ and Rust
In order to being able to use data written in both languages, the rust structures must be marked
as `#[repr(C)]`, also, type size must be taken into account. 

For example, a `char` in rust has a
size of 4 bytes, in C/C++ it has a size of 1 byte, so that must be taken into account, because if structure
sizes and/or alignments aren't the same, the data won't be correct.

In this example, we'll use C++ to write a struct into a file, then we'll read it from Rust.

C++ code:

```cpp
#include<iostream>
#include<fstream>

using namespace std;

struct SomeData {
    unsigned int a;
    unsigned long long b;
    char msg[13];
};

int main() {
    // Let's assume this operation won't fail.
    ofstream file("myfile.bin");
    
    // Create an instance to write to the file.
    SomeData instance = {
        128,
        256,
        "Hello World!"
    };
    
    // Write the struct to the file.
    file.write((const char*) &instance, sizeof(SomeData));

    return 0;
}
```

Rust code:

```rust
use binext::BinaryRead;
use std::{io, fs::OpenOptions};

#[repr(C)]
#[derive(Debug)]
struct SomeData {
    a: u32,
    b: u64,
    // u8 is the equivalent in rust of C's char
    msg: [u8; 13]
}

fn main() -> io::Result<()> {
    // Open the file in read mode.
    let mut file = OpenOptions::new()
        .read(true)
        .open("myfile.bin")?;

    // Read the structure from the file.
    let data = file.read_binary::<SomeData>()?;

    println!("Data: {data:?}");

    let message = String::from_utf8(data.msg.to_vec()).unwrap();
    println!("{message}");

    Ok(())
}
```

[Read]: https://doc.rust-lang.org/std/io/trait.Read.html
[Write]: https://doc.rust-lang.org/std/io/trait.Write.html
