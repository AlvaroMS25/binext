//! binext is a library that aims to make easier working with binary buffers and structs like you
//! would in C.
//!
//! This library provides safe interfaces to write/read structs from [Read]/[Write] binary sources.
//!
//! If used along with #\[repr(C)], this crate allows to read/write binary structures between C/C++
//! and Rust.
//!
//! Reading from/to a buffer is as easy as the following:
//!
//! ```rust
//! use binext::{BinaryWrite, BinaryRead};
//! use std::{io, fs::OpenOptions};
//!
//! #[derive(Debug, Default)] // This is just to avoid creating all fields manually.
//! struct MyStruct {
//!     a: u32,
//!     b: i16,
//!     c: char
//! }
//!
//! fn main() -> io::Result<()> {
//!     let mut write_file = OpenOptions::new()
//!         .write(true)
//!         .create(true)
//!         .truncate(true)
//!         .open("somefile.bin")?;
//!
//!     let item = MyStruct {
//!         a: 256,
//!         ..Default::default()
//!     };
//!
//!     // Write the item into the file.
//!     write_file.write_binary(&item)?;
//!
//!     // Drop both the file and the item.
//!     drop(write_file);
//!     drop(item);
//!
//!     let mut read_file = OpenOptions::new()
//!         .read(true)
//!         .open("somefile.bin")?;
//!
//!     // This item has the same data as the previous one.
//!     let new_item = read_file.read_binary::<MyStruct>()?;
//!
//!     println!("{new_item:?}");
//!
//!     Ok(())
//! }
//!
//! ```
//!
//! However, it is not limited to files, any type implementing [Read]/[Write] implements
//! [BinaryRead]/[BinaryWrite] respectively.
//!
//! For example, we could use a `Vec<u8>` which is [Write], to write in it and then wrap it with a
//! `Cursor` to make it [Read] and read it's contents into another instance:
//!
//! ```rust
//! use binext::{BinaryWrite, BinaryRead};
//! use std::io::{self, Cursor};
//!
//! #[derive(Debug)]
//! struct MyStruct {
//!     a: u32,
//!     b: i16,
//!     c: char
//! }
//!
//! fn main() -> io::Result<()> {
//!     let item = MyStruct {
//!         a: 256,
//!         c: '.',
//!         b: 128
//!     };
//!
//!     let mut buffer = Vec::new();
//!
//!     // Write struct into the buffer.
//!     buffer.write_binary(&item)?;
//!     drop(item);
//!
//!     // In order to make a Vec<u8> Read, a cursor must be used.
//!     let mut  cursor = Cursor::new(buffer);
//!
//!     // Now we read the contents of the previous instance back.
//!     let recovered = cursor.read_binary::<MyStruct>()?;
//!
//!     println!("{recovered:?}");
//!
//!     Ok(())
//! }
//!
//! ```
//!
//! [Read]: std::io::Read
//! [Write]: std::io::Write
//! [BinaryRead]: BinaryRead
//! [BinaryWrite]: BinaryWrite
//!

#[cfg(test)]
mod tests;

use std::{alloc::{alloc, Layout}, io::{self, Write, Read}, mem::size_of, slice};

/// The BinaryRead trait allows for reading data structures out of binary sources.
///
/// # Examples
///
/// ```rust
/// struct MyStruct {
///     some: char,
///     // fields
/// }
///
/// use binext::BinaryRead;
/// use std::{io, fs};
///
/// fn main() -> io::Result<()> {
///     let mut file = fs::File::open("myfile.bin")?;
///
///     // This returns an instance of the file that was in the file.
///     // Please note that for this method to work properly, the data must be written beforehand
///     // into the file.
///     let instance = file.read_binary::<MyStruct>()?;
///
///     Ok(())
/// }
/// ```
///
/// [Read]: Read
///
pub trait BinaryRead: Read {
    /// Reads from a binary source and converts the bytes into the specified structure, returning
    /// a `Box`ed structure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct MyStruct {
    ///     some: char,
    ///     // fields
    /// }
    ///
    /// use binext::BinaryRead;
    /// use std::{io, fs};
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut file = fs::File::open("myfile.bin")?;
    ///
    ///     // This returns an instance of the file that was in the file.
    ///     // Please note that for this method to work properly, the data must be written beforehand
    ///     // into the file.
    ///     let instance = file.read_binary_boxed::<MyStruct>()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    fn read_binary_boxed<T>(&mut self) -> io::Result<Box<T>> {
        Ok(unsafe {
            // Allocate the memory using the global allocator, so it can be Boxed later.
            let ptr = alloc(Layout::new::<T>());

            // SAFETY: all needed conditions for this not to be UB are satisfied, see
            // slice::from_raw_parts_mut to see them.
            let slice = slice::from_raw_parts_mut(ptr, size_of::<T>());

            self.read_exact(slice)?;

            // SAFETY: The pointer has been written to, since it has not been freed, it is still valid.
            Box::from_raw(ptr as *mut T)
        })
    }

    /// Reads from a binary source and converts the bytes into the specified structure.
    ///
    /// # Examples
    ///
    /// ```rust
    /// struct MyStruct {
    ///     some: char,
    ///     // fields
    /// }
    ///
    /// use binext::BinaryRead;
    /// use std::{io, fs};
    ///
    /// fn main() -> io::Result<()> {
    ///     let mut file = fs::File::open("myfile.bin")?;
    ///
    ///     // This returns an instance of the file that was in the file.
    ///     // Please note that for this method to work properly, the data must be written beforehand
    ///     // into the file.
    ///     let instance = file.read_binary::<MyStruct>()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    fn read_binary<T>(&mut self) -> io::Result<T> {
        self.read_binary_boxed()
            .map(|boxed| *boxed)
    }
}

/// The BinaryRead trait allows for writing data structures into binary [Write] sources.
///
/// # Examples
///
/// ```rust
/// #[derive(Default)]
/// struct MyStruct {
///     some: char,
///     // fields
/// }
///
/// use binext::BinaryWrite;
/// use std::{io, fs};
///
/// fn main() -> io::Result<()> {
///     use std::sync::Mutex;
///     let mut file = fs::OpenOptions::new()
///         .write(true)
///         .create(true)
///         .truncate(true)
///         .open("myfile.bin")?;
///
///     let instance = MyStruct::default();
///
///     // Write the struct into the file, this way, later when read, it can be recovered.
///     file.write_binary(&instance)?;
///
///     Ok(())
/// }
/// ```
///
/// [Write]: Write
///
pub trait BinaryWrite: Write {
    /// Writes into a binary source the provided struct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// #[derive(Default)]
    /// struct MyStruct {
    ///     some: char,
    ///     // fields
    /// }
    ///
    /// use binext::BinaryWrite;
    /// use std::{io, fs};
    ///
    /// fn main() -> io::Result<()> {
    ///     use std::sync::Mutex;
    ///     let mut file = fs::OpenOptions::new()
    ///         .write(true)
    ///         .create(true)
    ///         .truncate(true)
    ///         .open("myfile.bin")?;
    ///
    ///     let instance = MyStruct::default();
    ///
    ///     // Write the struct into the file, this way, later when read, it can be recovered.
    ///     file.write_binary(&instance)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn write_binary<T>(&mut self, item: &T) -> io::Result<()> {
        let ptr = item as *const T as *mut u8;

        // SAFETY: all needed conditions for this not to be UB are satisfied, see
        // slice::from_raw_parts to see them.
        let buf = unsafe {
            slice::from_raw_parts(ptr, size_of::<T>())
        };

        self.write(buf)?;
        Ok(())
    }
}

impl<I: Write> BinaryWrite for I {}

impl<I: Read> BinaryRead for I {}
