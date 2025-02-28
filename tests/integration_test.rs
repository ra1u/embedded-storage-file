use embedded_storage::{ReadStorage, Storage};
use embedded_storage_async::nor_flash::NorFlash;
use embedded_storage_async::nor_flash::ReadNorFlash;
use embedded_storage_file::{NorMemoryAsync, NorMemoryInFile, NorMemoryInram};
use rand::Rng;
use rand::SeedableRng;
use tokio;

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

// Generate a random vector of bytes given a size and a seed.
fn rand_vector(size: usize, seed: u64) -> Vec<u8> {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
    let mut v = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(rng.random());
    }
    v
}
