use embedded_storage::nor_flash::NorFlash as NorFlashSync;
use embedded_storage::nor_flash::ReadNorFlash as ReadNorFlashSync;
use embedded_storage::nor_flash::RmwMultiwriteNorFlashStorage;
use embedded_storage::{ReadStorage, Storage};
use embedded_storage_async::nor_flash::NorFlash as NorFlashAsync;
use embedded_storage_async::nor_flash::ReadNorFlash as ReadNorFlashAsync;
use embedded_storage_file::{NorMemoryAsync, NorMemoryInFile, NorMemoryInram};
use rand::RngExt;
use rand::SeedableRng;

#[test]
fn test_inmemory() {
    let mem_len = 4096_usize;
    let nor = NorMemoryInram::<256, 256, 256>::new(mem_len);
    let mut storage = nor.storage();

    let vin = rand_vector(mem_len, 7);
    storage.write(0, &vin).unwrap();
    let mut vread = vec![0u8; mem_len];
    storage.read(0, &mut vread).unwrap();
    assert_eq!(vin, vread);
}

#[test]
fn test_infile() {
    let path = "tests/test1.nor";
    let seed = rand::rng().random(); // random seed that is shared between write and read
    let mem_len = 4096_usize;
    {
        let nor = NorMemoryInFile::<256, 256, 256>::new(path, 4096).unwrap();
        let mut storage = nor.storage();
        let vrand = rand_vector(mem_len, seed);
        storage.write(0, &vrand).unwrap();
        let mut vread = vec![0u8; mem_len];
        storage.read(0, &mut vread).unwrap();
        assert_eq!(vread, vread);
    }
    {
        let nor2 = NorMemoryInFile::<256, 256, 256>::new(path, 4096).unwrap();
        let mut storage = nor2.storage();
        let mut vread = vec![0u8; mem_len];
        storage.read(0, &mut vread).unwrap();
        let vin = rand_vector(mem_len, seed); // same that was written
        assert_eq!(vin, vread);
    }
    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn test_async_mem() {
    let mem_len = 4096_usize;
    let nor = NorMemoryInram::<256, 256, 256>::new(mem_len);
    let mut anor = NorMemoryAsync::new(nor);
    let vin = rand_vector(mem_len, rand::rng().random());
    anor.write(0, &vin).await.unwrap();
    let mut vread = vec![0u8; mem_len];
    anor.read(0, &mut vread).await.unwrap();
    assert_eq!(vin, vread);
}

#[tokio::test]
async fn test_async_infile() {
    let mem_len = 4096_usize;
    let path = "tests/test2.nor";
    let nor = NorMemoryInFile::<256, 256, 256>::new(path, 4096).unwrap();
    let mut anor = NorMemoryAsync::new(nor);
    let vin = rand_vector(mem_len, rand::rng().random());
    anor.write(0, &vin).await.unwrap();
    let mut vread = vec![0u8; mem_len];
    anor.read(0, &mut vread).await.unwrap();
    assert_eq!(vin, vread);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_multiwrite_storage_sync() {
    let mem_len = 4096_usize;
    let nor = NorMemoryInram::<256, 256, 256>::new(mem_len);
    let mut merge_buffer = vec![0u8; 256];
    let mut storage = RmwMultiwriteNorFlashStorage::new(nor, &mut merge_buffer);

    let vin = rand_vector(mem_len, 11);
    storage.write(0, &vin).unwrap();
    let mut vread = vec![0u8; mem_len];
    storage.read(0, &mut vread).unwrap();
    assert_eq!(vin, vread);

    // overwrite with different data; RMW storage must erase where needed
    let vin2 = rand_vector(mem_len, 12);
    storage.write(0, &vin2).unwrap();
    storage.read(0, &mut vread).unwrap();
    assert_eq!(vin2, vread);
}

#[test]
fn test_fresh_inram_is_erased() {
    let mem_len = 4096_usize;
    let mut nor = NorMemoryInram::<256, 256, 256>::new(mem_len);
    let mut vread = vec![0u8; mem_len];
    nor.read(0, &mut vread).unwrap();
    assert!(vread.iter().all(|&b| b == 0xFF));
}

#[test]
fn test_write_and_semantics() {
    let mut nor = NorMemoryInram::<256, 256, 256>::new(4096);
    let block = 256_u32; // second erase block
    let mut vread = vec![0u8; 256];

    // fresh device is erased: write is stored as-is
    nor.write(block, &[0xF0u8; 256]).unwrap();
    nor.read(block, &mut vread).unwrap();
    assert!(vread.iter().all(|&b| b == 0xF0));

    // second write to the same block: result is the AND of old and new
    nor.write(block, &[0x0Fu8; 256]).unwrap();
    nor.read(block, &mut vread).unwrap();
    assert!(vread.iter().all(|&b| b == 0x00));

    // bits can only go back to 1 through erase
    nor.write(block, &[0xFFu8; 256]).unwrap();
    nor.read(block, &mut vread).unwrap();
    assert!(vread.iter().all(|&b| b == 0x00));
    nor.erase(block, block + 256).unwrap();
    nor.read(block, &mut vread).unwrap();
    assert!(vread.iter().all(|&b| b == 0xFF));

    // neighbouring blocks untouched
    nor.read(0, &mut vread).unwrap();
    assert!(vread.iter().all(|&b| b == 0xFF));
}

// Generate a random vector of bytes given a size and a seed.
fn rand_vector(size: usize, seed: u64) -> Vec<u8> {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
    let mut v = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(rng.random());
    }
    v
}
