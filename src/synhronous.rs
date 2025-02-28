use embedded_storage;
use embedded_storage::nor_flash::NorFlash;
use embedded_storage::nor_flash::ReadNorFlash;
use embedded_storage::nor_flash::RmwNorFlashStorage;

use std::usize;

// This module provides an implementation of a NOR flash memory storage
// using a buffer backend. It includes structures and traits to handle
// reading, writing, and erasing operations on the NOR flash memory.

/// Error type for NOR flash operations.
pub type Error = embedded_storage::nor_flash::NorFlashErrorKind;

/// Structure representing a NOR flash memory with a buffer backend.
pub struct NorMemory<
    B: BufferBackend,
    const READ_SIZE: usize,
    const WRITE_SIZE: usize,
    const ERASE_SIZE: usize,
> {
    pub(crate) buffer: B,
}

impl<B: BufferBackend, const READ_SIZE: usize, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    NorMemory<B, READ_SIZE, WRITE_SIZE, ERASE_SIZE>
{
    /// Converts the NOR memory into a NOR storage.
    pub fn storage(self) -> NorStorage<B, READ_SIZE, WRITE_SIZE, ERASE_SIZE> {
        NorStorage {
            wrapper: self,
            merge_buffer: vec![0u8; ERASE_SIZE],
        }
    }
}

/// Trait representing a buffer backend for the NOR flash memory.
pub trait BufferBackend {
    /// Executes a closure with a slice of data from the buffer.
    fn with_data<F>(&self, from: usize, to: usize, f: F) -> Result<(), Error>
    where
        F: FnMut(&[u8]) -> Result<(), Error>;

    /// Executes a closure with a mutable slice of data from the buffer.
    fn with_data_mut<F>(&mut self, from: usize, to: usize, f: F) -> Result<(), Error>
    where
        F: FnMut(&mut [u8]) -> Result<(), Error>;

    /// Returns the size of the buffer.
    fn size(&self) -> usize;
}

/// Structure representing a NOR storage with a buffer backend.
pub struct NorStorage<
    B: BufferBackend,
    const READ_SIZE: usize,
    const WRITE_SIZE: usize,
    const ERASE_SIZE: usize,
> {
    wrapper: NorMemory<B, READ_SIZE, WRITE_SIZE, ERASE_SIZE>,
    merge_buffer: Vec<u8>,
}

impl<B: BufferBackend, const READ_SIZE: usize, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    NorStorage<B, READ_SIZE, WRITE_SIZE, ERASE_SIZE>
{
    /// Converts the NOR storage back into a NOR memory.
    pub fn nor_flash(self) -> NorMemory<B, READ_SIZE, WRITE_SIZE, ERASE_SIZE> {
        self.wrapper
    }
}

impl<B: BufferBackend, const READ_SIZE: usize, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    embedded_storage::nor_flash::ErrorType for NorMemory<B, READ_SIZE, WRITE_SIZE, ERASE_SIZE>
{
    type Error = embedded_storage::nor_flash::NorFlashErrorKind;
}

impl<B: BufferBackend, const READ_SIZE: usize, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    ReadNorFlash for NorMemory<B, READ_SIZE, WRITE_SIZE, ERASE_SIZE>
{
    const READ_SIZE: usize = READ_SIZE;

    /// Reads data from the NOR flash memory into the provided buffer.
    fn read(&mut self, address: u32, buf: &mut [u8]) -> Result<(), Error> {
        if address as usize + buf.len() > self.capacity() {
            return Err(embedded_storage::nor_flash::NorFlashErrorKind::OutOfBounds);
        }
        let start = address as usize;
        let end = start + buf.len();
        self.buffer.with_data(start, end, |data| {
            buf.copy_from_slice(data);
            Ok(())
        })
    }

    /// Returns the capacity of the NOR flash memory.
    fn capacity(&self) -> usize {
        self.buffer.size()
    }
}

impl<B: BufferBackend, const READ_SIZE: usize, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    NorFlash for NorMemory<B, READ_SIZE, WRITE_SIZE, ERASE_SIZE>
{
    const WRITE_SIZE: usize = WRITE_SIZE;
    const ERASE_SIZE: usize = ERASE_SIZE;

    /// Erases data in the NOR flash memory from the specified range.
    fn erase(&mut self, from: u32, to: u32) -> Result<(), Error> {
        if from as usize % ERASE_SIZE != 0 {
            return Err(Error::NotAligned);
        }
        if to as usize % ERASE_SIZE != 0 {
            return Err(Error::NotAligned);
        }
        if to < from {
            return Err(Error::OutOfBounds);
        }
        if to as usize > NorMemory::capacity(self) {
            return Err(Error::OutOfBounds);
        }
        if from == to {
            // nothing to do
            return Ok(());
        }
        self.buffer
            .with_data_mut(from as usize, to as usize, |data| {
                let len = data.len();
                if from as usize + len != to as usize {
                    // this is not expected
                    return Err(Error::OutOfBounds);
                }
                for i in 0..len {
                    data[i] = 0xFF;
                }
                Ok(())
            })
    }

    /// Writes data to the NOR flash memory at the specified offset.
    fn write(&mut self, offset: u32, data: &[u8]) -> Result<(), Error> {
        let cap = self.capacity();
        if offset as usize + data.len() > cap {
            return Err(Error::OutOfBounds);
        }
        self.buffer
            .with_data_mut(offset as usize, offset as usize + data.len(), |buf| {
                buf.copy_from_slice(data);
                Ok(())
            })
    }
}

impl<B: BufferBackend, const READ_SIZE: usize, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    embedded_storage::ReadStorage for NorStorage<B, READ_SIZE, WRITE_SIZE, ERASE_SIZE>
{
    type Error = embedded_storage::nor_flash::NorFlashErrorKind;

    /// Reads data from the NOR storage into the provided buffer.
    fn read(&mut self, address: u32, buf: &mut [u8]) -> Result<(), Error> {
        let mut storage = RmwNorFlashStorage::new(&mut self.wrapper, &mut self.merge_buffer[..]);
        storage.read(address, buf)
    }

    /// Returns the capacity of the NOR storage.
    fn capacity(&self) -> usize {
        self.wrapper.capacity()
    }
}

impl<B: BufferBackend, const READ_SIZE: usize, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    embedded_storage::Storage for NorStorage<B, READ_SIZE, WRITE_SIZE, ERASE_SIZE>
{
    /// Writes data to the NOR storage at the specified address.
    fn write(&mut self, address: u32, buf: &[u8]) -> Result<(), Error> {
        let mut storage = RmwNorFlashStorage::new(&mut self.wrapper, &mut self.merge_buffer[..]);
        storage.write(address, buf)
    }
}
