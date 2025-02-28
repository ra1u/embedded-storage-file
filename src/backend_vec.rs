use crate::synhronous::{NorMemory, BufferBackend, Error};
use std::vec::Vec;

pub type NorMemoryInram<const READ_SIZE: usize, const WRITE_SIZE: usize, const ERASE_SIZE: usize> =
    NorMemory<Vec<u8>, READ_SIZE, WRITE_SIZE, ERASE_SIZE>;

impl<const READ_SIZE: usize, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    NorMemoryInram<READ_SIZE, WRITE_SIZE, ERASE_SIZE>
{
    pub fn new(size: usize) -> Self {
        Self {
            buffer: vec![0; size],
        }
    }
}

impl BufferBackend for Vec<u8> {
    fn with_data<F>(&self, from: usize, to: usize, mut f: F) -> Result<(), Error>
    where
        F: FnMut(&[u8]) -> Result<(), Error>,
    {
        if to > self.len() {
            return Err(Error::OutOfBounds);
        }
        if from > to {
            return Err(Error::OutOfBounds);
        }
        let r = &self[from..to];
        f(r)
    }

    fn with_data_mut<F>(&mut self, from: usize, to: usize, mut f: F) -> Result<(), Error>
    where
        F: FnMut(&mut [u8]) -> Result<(), Error>,
    {
        if to > self.len() {
            return Err(Error::OutOfBounds);
        }
        if from > to {
            return Err(Error::OutOfBounds);
        }
        let r = &mut self[from..to];
        f(r)
    }

    fn size(&self) -> usize {
        self.len()
    }
}