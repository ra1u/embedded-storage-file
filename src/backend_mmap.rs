use crate::synhronous::{BufferBackend, Error, NorMemory};
use memmap2;

pub struct MmapFile {
    pub file: std::fs::File,
    pub mmap: memmap2::MmapMut,
}

pub type NorMemoryInFile<const READ_SIZE: usize, const WRITE_SIZE: usize, const ERASE_SIZE: usize> =
    NorMemory<MmapFile, READ_SIZE, WRITE_SIZE, ERASE_SIZE>;

impl<const READ_SIZE: usize, const WRITE_SIZE: usize, const ERASE_SIZE: usize>
    NorMemoryInFile<READ_SIZE, WRITE_SIZE, ERASE_SIZE>
{
    pub fn new<P: std::convert::AsRef<std::path::Path>>(
        path: P,
        size: usize,
    ) -> Result<Self, std::io::Error> {
        let is_new = !path.as_ref().exists();
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        file.set_len(size as u64)?;
        let mut mmap = unsafe { memmap2::MmapOptions::new().map_mut(&file)? };
        if is_new {
            for i in 0..size {
                mmap[i] = 0xFF;
            }
        }
        Ok(Self {
            buffer: MmapFile { file, mmap },
        })
    }

    pub fn new_from_mmap(mmap: MmapFile) -> Self {
        Self { buffer: mmap }
    }
}

impl BufferBackend for MmapFile {
    fn with_data<F>(&self, from: usize, to: usize, mut f: F) -> Result<(), Error>
    where
        F: FnMut(&[u8]) -> Result<(), Error>,
    {
        if to > self.size() {
            return Err(Error::OutOfBounds);
        }
        if from > to {
            return Err(Error::OutOfBounds);
        }
        let b = &self.mmap[from..to];
        f(b)
    }

    fn with_data_mut<F>(&mut self, from: usize, to: usize, mut f: F) -> Result<(), Error>
    where
        F: FnMut(&mut [u8]) -> Result<(), Error>,
    {
        if to > self.size() {
            return Err(Error::OutOfBounds);
        }
        if from > to {
            return Err(Error::OutOfBounds);
        }
        let b = &mut self.mmap[from..to];
        f(b)
    }

    fn size(&self) -> usize {
        self.mmap.len()
    }
}
