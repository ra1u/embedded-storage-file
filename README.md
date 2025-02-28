
# embedded-storage-file

Library exposes traits from [embedded_storage] and [embedded_storage_async]. 
Exposed interface works as NOR flash where file is used as a storage medium.
Under the hood we use memory mapped file from [memmap2].

Interface for using just in-memory (aka RAM) is also available.

Main use case is for testing and development purpose, so that we can mock NOR storage.


## In file example

```
use embedded_storage::{ReadStorage, Storage};
use embedded_storage_file::NorMemoryInFile;

let path = "tests/test1.nor";
let capacity = 4096;
// create nor interface that implements traits from embedded-storage::nor_flash
let nor = NorMemoryInFile::<
    256, // READ_SIZE
    256, // WRITE_SIZE
    256, // ERASE_SIZE
>::new(path, capacity)
.unwrap();
// convert to storage that implements embedded-storage::Storage
let mut storage = nor.storage();
// write & read 512 bytes from offset 128
let vin = vec![0x55u8; 512];
storage.write(128, &vin).unwrap();
let mut vread = vec![0u8; 512];
storage.read(128, &mut vread).unwrap();
assert_eq!(vin, vread);
std::fs::remove_file(path).unwrap();
```

# other examples

For other usecase examples check `tests/integration_test.rs`