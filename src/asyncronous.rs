use embedded_storage::nor_flash::ErrorType as ErrorTypeSync;
use embedded_storage::nor_flash::MultiwriteNorFlash as MultiwriteNorFlashSync;
use embedded_storage::nor_flash::NorFlash as NorFlashSync;
use embedded_storage::nor_flash::ReadNorFlash as ReadNorFlashSync;
use embedded_storage_async::nor_flash::ErrorType as ErrorTypeAsync;
use embedded_storage_async::nor_flash::MultiwriteNorFlash as MultiwriteNorFlashAsync;
use embedded_storage_async::nor_flash::NorFlash as NorFlashAsync;
use embedded_storage_async::nor_flash::ReadNorFlash as ReadNorFlashAsync;

pub struct NorMemoryAsync<NMS> {
    nor_memory_sync: NMS,
}

/// Asynchronous NOR flash memory interface from [`embedded_storage_async::nor_flash`].  
/// it is a wrapper around a synchronous NOR flash memory.  
/// Note: all operations are executed synchronously, prentending to be asynchronous.  
impl<NMS> NorMemoryAsync<NMS> {
    /// given a synchronous NOR flash memory, returns an asynchronous NOR flash memory.
    pub fn new(nor_memory_sync: NMS) -> Self {
        Self { nor_memory_sync }
    }

    /// returns the synchronous NOR flash memory back.
    pub fn get_sync(self) -> NMS {
        self.nor_memory_sync
    }
}

impl<NMS> ErrorTypeAsync for NorMemoryAsync<NMS>
where
    NMS: ErrorTypeSync,
{
    type Error = NMS::Error;
}

impl<NMS> ReadNorFlashAsync for NorMemoryAsync<NMS>
where
    NMS: ReadNorFlashSync,
{
    const READ_SIZE: usize = NMS::READ_SIZE;

    async fn read(&mut self, address: u32, buf: &mut [u8]) -> Result<(), Self::Error> {
        async { self.nor_memory_sync.read(address, buf) }.await
    }

    fn capacity(&self) -> usize {
        self.nor_memory_sync.capacity()
    }
}

impl<NMS> NorFlashAsync for NorMemoryAsync<NMS>
where
    NMS: NorFlashSync,
{
    const WRITE_SIZE: usize = NMS::WRITE_SIZE;
    const ERASE_SIZE: usize = NMS::ERASE_SIZE;

    async fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        async { self.nor_memory_sync.erase(from, to) }.await
    }

    async fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        async { self.nor_memory_sync.write(offset, bytes) }.await
    }
}

impl<NMS> MultiwriteNorFlashAsync for NorMemoryAsync<NMS> where NMS: MultiwriteNorFlashSync {}
