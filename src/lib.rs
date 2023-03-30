#[cfg(test)]
mod tests;


use std::{alloc::{alloc, Layout}, io::{self, Write, Read}, mem::size_of, slice};

pub trait BinaryRead: Read {
    fn read_binary<T>(&mut self) -> io::Result<T> {
        Ok(unsafe {
            // Allocate the memory using the global allocator, so it can be Boxed later.
            let ptr = alloc(Layout::new::<T>());

            // SAFETY: all needed conditions for this not to be UB are satisfied, see
            // slice::from_raw_parts_mut to see them.
            let slice = slice::from_raw_parts_mut(ptr, size_of::<T>());

            self.read_exact(slice)?;

            // SAFETY: The pointer has been written to, since it has not been freed, it is still valid.
            *Box::from_raw(ptr as *mut T)
        })
    }
}

pub trait BinaryWrite: Write {
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
