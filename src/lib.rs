#[cfg(test)]
mod tests;


use std::{alloc::{alloc, Layout}, io::{self, Write, Read}, mem::size_of, slice};

pub trait BinaryRead {
    fn read<T>(&mut self) -> io::Result<T>;
}

pub trait BinaryWrite {
    fn write<T>(&mut self, item: &T) -> io::Result<()>;
}

impl<I: Write> BinaryWrite for I {
    fn write<T>(&mut self, item: &T) -> io::Result<()> {
        let ptr = item as *const T as *mut u8;
        let buf = unsafe {
            slice::from_raw_parts(ptr, size_of::<T>())
        };

        self.write(buf)?;
        Ok(())
    }
}

impl<I: Read> BinaryRead for I {
    fn read<T>(&mut self) -> io::Result<T> {
        Ok(unsafe {
            let ptr = alloc(Layout::new::<T>());

            let slice = slice::from_raw_parts_mut(ptr, size_of::<T>());

            self.read_exact(slice)?;

            *Box::from_raw(ptr as *mut T)
        })
    }
}
